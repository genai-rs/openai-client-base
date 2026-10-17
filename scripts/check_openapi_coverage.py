#!/usr/bin/env python3
"""Fail closed unless generated Rust APIs account for every OpenAPI path operation.

Webhook operations are audited separately from normal ``paths`` operations. A
webhook can never satisfy the inventory for a missing HTTP method/path pair.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from collections import defaultdict
from pathlib import Path
from typing import Any

import yaml

HTTP_METHODS = {"get": "GET", "put": "PUT", "post": "POST", "delete": "DELETE",
                "options": "OPTIONS", "head": "HEAD", "patch": "PATCH", "trace": "TRACE"}


def normalize_path(path: str) -> str:
    """Normalize path parameter names while retaining path shape and literals."""
    return re.sub(r"\{[^{}]*\}", "{}", path)


def rust_operation_name(operation_id: str) -> str:
    """Match the Rust generator's usual snake-case operation naming."""
    value = operation_id.replace("-", "_").replace(".", "_")
    value = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", value)
    value = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", value)
    return re.sub(r"_+", "_", value).strip("_").lower()


def resolve_local_reference(document: dict[str, Any], reference: str) -> Any:
    """Resolve a local JSON Pointer reference or raise an actionable error."""
    if not reference.startswith("#/"):
        raise ValueError(f"only local JSON Pointer references are supported: {reference!r}")
    target: Any = document
    for token in reference[2:].split("/"):
        token = token.replace("~1", "/").replace("~0", "~")
        if isinstance(target, dict) and token in target:
            target = target[token]
        elif isinstance(target, list) and token.isdigit() and int(token) < len(target):
            target = target[int(token)]
        else:
            raise ValueError(f"reference target does not exist: {reference!r}")
    return target


def resolve_path_item(document: dict[str, Any], item: Any, path: str,
                      reference_chain: tuple[str, ...] = ()) -> tuple[dict[str, Any] | None, str | None]:
    if not isinstance(item, dict):
        return None, "Path Item must be an object"
    if "$ref" not in item:
        return item, None
    reference = item["$ref"]
    if not isinstance(reference, str):
        return None, f"Path Item $ref must be a string for {path!r}"
    if len(item) != 1:
        return None, f"Path Item $ref siblings are unsupported for {reference!r}"
    if reference in reference_chain:
        cycle = " -> ".join((*reference_chain, reference))
        return None, f"cyclic Path Item reference: {cycle}"
    try:
        target = resolve_local_reference(document, reference)
    except ValueError as error:
        return None, str(error)
    return resolve_path_item(document, target, path, (*reference_chain, reference))


def input_inventory(spec: dict[str, Any]) -> tuple[list[dict[str, str]], list[dict[str, str]]]:
    normal: list[dict[str, str]] = []
    invalid: list[dict[str, str]] = []
    paths = spec.get("paths", {})
    if not isinstance(paths, dict):
        return normal, [{"error": "OpenAPI paths must be an object"}]
    for path, item in paths.items():
        if not isinstance(path, str):
            invalid.append({"error": "Invalid path item", "path": str(path)})
            continue
        resolved, error = resolve_path_item(spec, item, path)
        if error:
            invalid.append({"error": error, "path": path})
            continue
        for method, operation in resolved.items():
            if method not in HTTP_METHODS:
                continue
            if not isinstance(operation, dict):
                invalid.append({"error": "Operation must be an object", "method": method, "path": path})
                continue
            operation_id = operation.get("operationId")
            if not isinstance(operation_id, str) or not operation_id:
                invalid.append({"error": "Path operation lacks operationId", "method": method, "path": path})
                continue
            normal.append({"operation_id": operation_id, "method": HTTP_METHODS[method],
                           "path": path, "normalized_path": normalize_path(path)})

    keys = [(entry["method"], entry["normalized_path"]) for entry in normal]
    for key in sorted({key for key in keys if keys.count(key) > 1}):
        invalid.append({"error": "Duplicate method/path after parameter normalization",
                        "method": key[0], "path": key[1]})
    return normal, invalid


def extract_functions(apis_dir: Path) -> tuple[list[dict[str, str]], list[dict[str, str]]]:
    generated: list[dict[str, str]] = []
    unparsed: list[dict[str, str]] = []
    declaration = re.compile(r"(?m)^\s*pub\s+async\s+fn\s+(\w+)\s*\(")
    uri_pattern = re.compile(r"let\s+uri_str\s*=\s*format!\(\s*\"((?:\\.|[^\"\\])*)\"", re.S)
    method_pattern = re.compile(r"\.request\(\s*reqwest::Method::(\w+)")

    for file in sorted(apis_dir.glob("*_api.rs")):
        source = file.read_text(encoding="utf-8")
        functions = list(declaration.finditer(source))
        for index, match in enumerate(functions):
            end = functions[index + 1].start() if index + 1 < len(functions) else len(source)
            body = source[match.start():end]
            uri = uri_pattern.search(body)
            method = method_pattern.search(body)
            if not uri or not method:
                unparsed.append({"file": file.name, "function": match.group(1),
                                 "reason": "could not read reqwest method and format! URI"})
                continue
            template = bytes(uri.group(1), "utf-8").decode("unicode_escape")
            # Generated Rust operations form URIs from a base URL followed by a
            # literal endpoint. Convert path interpolation to the same wildcard
            # used for OpenAPI path parameter names.
            base = re.match(r"^\{(?:[A-Za-z_][A-Za-z0-9_]*|)\}", template)
            if not base:
                unparsed.append({"file": file.name, "function": match.group(1),
                                 "reason": f"unsupported URI format string {template!r}"})
                continue
            path = template[base.end():]
            path = path.replace("{{", "\0").replace("}}", "\1")
            path = re.sub(r"\{[^{}]*\}", "{}", path).replace("\0", "{").replace("\1", "}")
            if not path.startswith("/"):
                unparsed.append({"file": file.name, "function": match.group(1),
                                 "reason": f"URI has no absolute endpoint path: {template!r}"})
                continue
            generated.append({"module": file.stem, "function": match.group(1),
                              "method": method.group(1).upper(), "path": path,
                              "normalized_path": normalize_path(path)})
    return generated, unparsed


def webhook_inventory(spec: dict[str, Any]) -> list[dict[str, str]]:
    result: list[dict[str, str]] = []
    webhooks = spec.get("webhooks", {})
    if not isinstance(webhooks, dict):
        return [{"error": "OpenAPI webhooks must be an object"}]
    for name, item in webhooks.items():
        path_item, error = resolve_path_item(spec, item, str(name))
        if error:
            result.append({"error": error, "webhook": str(name)})
            continue
        for method, operation in path_item.items():
            if method not in HTTP_METHODS:
                continue
            operation_id = operation.get("operationId") if isinstance(operation, dict) else None
            if not isinstance(operation_id, str) or not operation_id:
                # OpenAPI allows webhook operations without operationId. The
                # Rust generator derives their method name from webhook key +
                # HTTP method (for example agent_environment_ready_post).
                operation_id = f"{name}_{method}"
                operation_id_source = "webhook-name-and-method"
            else:
                operation_id_source = "operationId"
            result.append({"operation_id": operation_id,
                           "operation_id_source": operation_id_source,
                           "function": rust_operation_name(operation_id),
                           "method": HTTP_METHODS[method], "webhook": str(name),
                           "path": f"/{name}", "normalized_path": normalize_path(f"/{name}")})
    return result


def audit(spec: dict[str, Any], apis_dir: Path, webhook_module: str) -> dict[str, Any]:
    expected, invalid = input_inventory(spec)
    generated, unparsed = extract_functions(apis_dir)
    webhooks = webhook_inventory(spec)
    invalid_webhooks = [entry for entry in webhooks if "error" in entry]
    webhook_names = {entry["function"] for entry in webhooks if "function" in entry}
    # Treat webhook operations as a distinct API surface even on the old
    # generator output where they share default_api.rs with normal endpoints.
    # Exclude by operation name so normal functions in that legacy module remain
    # eligible for matching.
    normal_generated = [entry for entry in generated if entry["function"] not in webhook_names]
    generated_by_key: dict[tuple[str, str], list[dict[str, str]]] = defaultdict(list)
    for entry in normal_generated:
        generated_by_key[(entry["method"], entry["normalized_path"])].append(entry)
    missing: list[dict[str, str]] = []
    ambiguous: list[dict[str, Any]] = []
    matches: list[dict[str, Any]] = []
    for operation in expected:
        key = (operation["method"], operation["normalized_path"])
        candidates = generated_by_key.get(key, [])
        if not candidates:
            missing.append(operation)
        elif len(candidates) > 1:
            ambiguous.append({"operation": operation, "generated": candidates})
        else:
            matches.append({"operation": operation, "generated": candidates[0]})

    missing_webhooks = []
    ambiguous_webhooks = []
    mismatched_webhooks = []
    for webhook in webhooks:
        if "function" not in webhook:
            continue
        webhook_matches = [entry for entry in generated if entry["function"] == webhook["function"]]
        correct_identity = [entry for entry in webhook_matches
                            if entry["method"] == webhook["method"] and
                            entry["normalized_path"] == webhook["normalized_path"]]
        wrong_identity = [entry for entry in webhook_matches if entry not in correct_identity]
        in_module = [entry for entry in correct_identity if entry["module"] == webhook_module]
        if not in_module:
            missing_webhooks.append(webhook)
        if wrong_identity:
            mismatched_webhooks.append({"webhook": webhook, "generated": wrong_identity})
        if len(correct_identity) > 1 or len(webhook_matches) > 1:
            ambiguous_webhooks.append({"webhook": webhook, "generated": webhook_matches})
    expected_keys = {(entry["method"], entry["normalized_path"]) for entry in expected}
    extra_generated = [entry for entry in normal_generated
                       if (entry["method"], entry["normalized_path"]) not in expected_keys]
    errors = bool(invalid or missing or ambiguous or unparsed or invalid_webhooks or
                  missing_webhooks or ambiguous_webhooks or mismatched_webhooks or extra_generated)
    return {"status": "fail" if errors else "pass", "expected_normal_operations": expected,
            "generated_api_functions": generated, "matched_normal_operations": matches,
            "missing_normal_operations": missing, "ambiguous_normal_operations": ambiguous,
            "invalid_input_operations": invalid, "unparsed_generated_functions": unparsed,
            "expected_webhooks": webhooks, "webhook_module": webhook_module,
            "invalid_webhook_operations": invalid_webhooks,
            "missing_webhook_functions": missing_webhooks,
            "ambiguous_webhook_functions": ambiguous_webhooks,
            "mismatched_webhook_functions": mismatched_webhooks,
            "unexpected_generated_operations": extra_generated}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("input_spec", type=Path)
    parser.add_argument("apis_dir", type=Path)
    parser.add_argument("--webhook-module", default="open_ai_client_base_webhooks_api")
    parser.add_argument("--report", type=Path)
    args = parser.parse_args()
    spec = yaml.safe_load(args.input_spec.read_text(encoding="utf-8"))
    if not isinstance(spec, dict):
        raise SystemExit("OpenAPI input is not an object")
    result = audit(spec, args.apis_dir, args.webhook_module)
    if args.report:
        args.report.parent.mkdir(parents=True, exist_ok=True)
        args.report.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"status": result["status"],
                      "normal_expected": len(result["expected_normal_operations"]),
                      "normal_matched": len(result["matched_normal_operations"]),
                      "normal_missing": len(result["missing_normal_operations"]),
                      "normal_ambiguous": len(result["ambiguous_normal_operations"]),
                      "webhooks_expected": len(result["expected_webhooks"]),
                      "webhooks_missing": len(result["missing_webhook_functions"]),
                      "webhooks_invalid": len(result["invalid_webhook_operations"]),
                      "webhooks_ambiguous": len(result["ambiguous_webhook_functions"]),
                      "webhooks_mismatched": len(result["mismatched_webhook_functions"]),
                      "unparsed": len(result["unparsed_generated_functions"])}, indent=2))
    if result["status"] != "pass":
        print("Full inventory written to the requested report; generation is not accepted.", file=sys.stderr)
        raise SystemExit(1)


if __name__ == "__main__":
    main()
