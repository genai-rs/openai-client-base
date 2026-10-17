#!/usr/bin/env python3
"""Read-only, deterministic OpenAPI and generated-file review evidence.

This is deliberately not a Rust parser or complete semver analysis. Snapshots
are review artifacts, never substitute inputs to the generation pipeline.
"""

import argparse
import hashlib
import json
import re
import sys
from pathlib import Path

import yaml

FORMAT_VERSION = 1
METHODS = {"get", "put", "post", "delete", "options", "head", "patch", "trace"}
UNORDERED = {"required", "enum", "allOf", "anyOf", "oneOf"}
SKIP = {"example", "examples", "default", "description", "summary", "externalDocs"}
DISCLAIMER = (
    "This is not complete Rust semver analysis. It compares declared OpenAPI "
    "operations, models, required fields and enum wire values, and generated "
    "Rust file hashes. It does not resolve schema composition or prove Rust "
    "signature, trait, feature, serialization or downstream compatibility. "
    "An empty report is not proof of compatibility. No rustdoc is invoked."
)


class OpenAPILoader(yaml.SafeLoader):
    """Keep JSON/OpenAPI strings such as on/off and dates as strings."""

    def construct_mapping(self, node, deep=False):
        self.flatten_mapping(node)
        mapping = {}
        for key_node, value_node in node.value:
            key = self.construct_object(key_node, deep=deep)
            if not isinstance(key, str):
                raise ValueError("OpenAPI object keys must be strings")
            if key in mapping:
                raise ValueError(f"duplicate OpenAPI key: {key}")
            mapping[key] = self.construct_object(value_node, deep=deep)
        return mapping


OpenAPILoader.yaml_implicit_resolvers = {
    key: [(tag, pattern) for tag, pattern in resolvers
          if tag not in {"tag:yaml.org,2002:bool", "tag:yaml.org,2002:timestamp"}]
    for key, resolvers in yaml.SafeLoader.yaml_implicit_resolvers.items()
}
OpenAPILoader.add_implicit_resolver(
    "tag:yaml.org,2002:bool", re.compile(r"^(?:true|false)$"), list("tf"))


def canonical(value):
    return json.dumps(value, sort_keys=True, ensure_ascii=False, separators=(",", ":"), allow_nan=False)


def normalize(value, key=""):
    """Ignore object ordering and set-like OpenAPI array ordering only."""
    if isinstance(value, dict):
        return {k: normalize(v, k) for k, v in sorted(value.items())}
    if isinstance(value, list):
        values = [normalize(v) for v in value]
        return sorted(values, key=canonical) if key in UNORDERED else values
    return value


def digest(data):
    return hashlib.sha256(data).hexdigest()


def pointer(parent, key):
    return parent + "/" + str(key).replace("~", "~0").replace("/", "~1")


def resolve(spec, ref):
    if not isinstance(ref, str) or not ref.startswith("#/"):
        raise ValueError(f"unsupported external reference: {ref!r}")
    node = spec
    try:
        for token in ref[2:].split("/"):
            token = token.replace("~1", "/").replace("~0", "~")
            node = node[int(token)] if isinstance(node, list) else node[token]
    except (KeyError, IndexError, TypeError, ValueError) as exc:
        raise ValueError(f"unresolved reference: {ref}") from exc
    return node


def inventory(spec):
    if not isinstance(spec, dict) or not str(spec.get("openapi", "")).startswith("3."):
        raise ValueError("expected an OpenAPI 3 document")
    paths = spec.get("paths")
    schemas = spec.get("components", {}).get("schemas", {})
    if not isinstance(paths, dict) or not isinstance(schemas, dict):
        raise ValueError("paths and components.schemas must be objects")
    spec = normalize(spec)
    required, enums = {}, {}

    def schema(node, location):
        if isinstance(node, bool):
            return
        if not isinstance(node, dict):
            raise ValueError(f"schema must be an object or boolean: {location}")
        if "$ref" in node:
            resolve(spec, node["$ref"])
        if "required" in node:
            if not isinstance(node["required"], list) or not all(isinstance(v, str) for v in node["required"]):
                raise ValueError(f"required must contain field names at {location}")
            required[location] = sorted(set(node["required"]))
        if "enum" in node:
            if not isinstance(node["enum"], list):
                raise ValueError(f"enum must be an array at {location}")
            # JSON strings preserve wire types: false, 0 and "0" differ.
            enums[location] = sorted({canonical(v) for v in node["enum"]})
        for key in ("properties", "patternProperties", "$defs", "dependentSchemas"):
            for name, value in node.get(key, {}).items():
                schema(value, pointer(pointer(location, key), name))
        for key in ("items", "additionalProperties", "unevaluatedProperties", "contains", "not", "if", "then", "else"):
            if key in node:
                schema(node[key], pointer(location, key))
        for key in ("allOf", "anyOf", "oneOf", "prefixItems"):
            for index, value in enumerate(node.get(key, [])):
                schema(value, pointer(pointer(location, key), index))

    def walk(node, location):
        if isinstance(node, dict):
            if "$ref" in node:
                resolve(spec, node["$ref"])
            for key, value in node.items():
                if location == "#/components" and key == "schemas":
                    for name, model in value.items():
                        schema(model, pointer("#/components/schemas", name))
                elif key == "schema":
                    schema(value, pointer(location, key))
                elif key not in SKIP and not key.startswith("x-"):
                    walk(value, pointer(location, key))
        elif isinstance(node, list):
            for index, value in enumerate(node):
                walk(value, pointer(location, index))

    walk(spec, "#")
    operations = {}
    for path, original_item in sorted(paths.items()):
        item = original_item
        seen = set()
        while isinstance(item, dict) and "$ref" in item:
            ref = item["$ref"]
            if ref in seen:
                raise ValueError(f"cyclic path reference: {ref}")
            seen.add(ref)
            # Ambiguous sibling overrides need explicit review, not guessing.
            if any(k in METHODS for k in item):
                raise ValueError(f"path reference has operation siblings: {path}")
            item = resolve(spec, ref)
        if not isinstance(item, dict):
            raise ValueError(f"path item must be an object: {path}")
        for method in sorted(METHODS & item.keys()):
            operation = item[method]
            if not isinstance(operation, dict):
                raise ValueError(f"operation must be an object: {method} {path}")
            operation_id = operation.get("operationId")
            if operation_id is not None and not isinstance(operation_id, str):
                raise ValueError(f"operationId must be a string: {method} {path}")
            operations[f"{method.upper()} {path}"] = operation_id
    return {"operations": operations, "models": sorted(schemas),
            "required": required, "enums": enums}


def snapshot(spec_path, generated_root, source_url, generator_version, revision):
    if not all(value.strip() for value in (source_url, generator_version, revision)):
        raise ValueError("source URL, generator version and revision are required")
    raw = spec_path.read_bytes()
    spec = yaml.load(raw, Loader=OpenAPILoader)
    surface = inventory(spec)
    files = {}
    for folder in ("src/apis", "src/models"):
        directory = generated_root / folder
        if not directory.is_dir():
            raise ValueError(f"missing generated directory: {directory}")
        paths = sorted(directory.rglob("*.rs"))
        if not paths:
            raise ValueError(f"no generated Rust files: {directory}")
        for path in paths:
            if path.is_symlink():
                raise ValueError(f"generated file must not be a symlink: {path}")
            files[path.relative_to(generated_root).as_posix()] = digest(path.read_bytes())
    return {
        "format_version": FORMAT_VERSION,
        "provenance": {"source_url": source_url, "input_sha256": digest(raw),
                       "generator_version": generator_version, "revision": revision},
        "surface": surface, "generated_files": files,
    }


def delta(before, after):
    return {"removed": sorted(set(before) - set(after)),
            "added": sorted(set(after) - set(before))}


def compare(before, after):
    for item in (before, after):
        if item.get("format_version") != FORMAT_VERSION:
            raise ValueError("unsupported snapshot format version")
        if not item["provenance"]["revision"] or not item["provenance"]["generator_version"]:
            raise ValueError("snapshot provenance is incomplete")
    old, new = before["surface"], after["surface"]
    changes = {"operations": delta(old["operations"], new["operations"]),
               "models": delta(old["models"], new["models"])}
    changes["operation_ids"] = [
        {"operation": key, "before": old["operations"][key], "after": new["operations"][key]}
        for key in sorted(old["operations"].keys() & new["operations"].keys())
        if old["operations"][key] != new["operations"][key]
    ]
    for category in ("required", "enums"):
        changes[category] = []
        for location in sorted(old[category].keys() | new[category].keys()):
            change = delta(old[category].get(location, []), new[category].get(location, []))
            if change["removed"] or change["added"]:
                changes[category].append({"location": location, **change})
    changes["generated_files"] = delta(before["generated_files"], after["generated_files"])
    changes["generated_files"]["changed"] = [
        name for name in sorted(before["generated_files"].keys() & after["generated_files"].keys())
        if before["generated_files"][name] != after["generated_files"][name]
    ]
    removals = (bool(changes["operations"]["removed"] or changes["models"]["removed"]
                     or changes["generated_files"]["removed"])
                or any(c["removed"] for c in changes["enums"]))
    changed = any(any(v.values()) if isinstance(v, dict) else bool(v) for v in changes.values())
    return {"format_version": FORMAT_VERSION, "baseline_sha": before["provenance"]["revision"],
            "before": before["provenance"], "after": after["provenance"],
            "before_snapshot_sha256": digest(canonical(before).encode()),
            "after_snapshot_sha256": digest(canonical(after).encode()),
            "review_required": changed, "removals_flagged": removals,
            "limitations": DISCLAIMER, "changes": changes}


def code(value):
    # Render untrusted upstream names as inert Markdown, including pipes/newlines.
    return "<code>" + str(value).replace("&", "&amp;").replace("<", "&lt;").replace(
        ">", "&gt;").replace("|", "&#124;").replace("\n", "&#10;").replace("\r", "&#13;") + "</code>"


def markdown(report):
    lines = ["# Generated API change report", "", report["limitations"], "",
             "**Explicit maintainer or designated senior reviewer approval required before merging.**"
             if report["review_required"] else "No covered changes detected; manual Rust API review remains required.",
             "", f"Removals flagged: **{'yes' if report['removals_flagged'] else 'no'}**.", "",
             "Record the reviewed head, affected surface, migration and version decision in the PR.", "",
             "## Provenance", "", "| Input | Before | After |", "| --- | --- | --- |"]
    for field in ("revision", "source_url", "input_sha256", "generator_version"):
        lines.append(f"| {field} | {code(report['before'][field])} | {code(report['after'][field])} |")
    lines.append(f"| snapshot_sha256 (canonical JSON) | {code(report['before_snapshot_sha256'])} | "
                 f"{code(report['after_snapshot_sha256'])} |")
    lines.extend(["", "## Changes", ""])
    for category, changes in report["changes"].items():
        lines.extend([f"### {category.replace('_', ' ').title()}", ""])
        entries = []
        if isinstance(changes, dict):
            entries = [f"- {kind}: {code(value)}" for kind, values in changes.items() for value in values]
        else:
            for item in changes:
                if "operation" in item:
                    entries.append(f"- {code(item['operation'])}: {code(item['before'])} → {code(item['after'])}")
                else:
                    for kind in ("removed", "added"):
                        if item[kind]:
                            entries.append(f"- {code(item['location'])} — {kind}: "
                                           + ", ".join(code(v) for v in item[kind]))
        lines.extend(entries or ["None."])
        lines.append("")
    return "\n".join(lines)


def write_json(path, value):
    path.write_text(json.dumps(value, indent=2, sort_keys=True, ensure_ascii=False) + "\n", encoding="utf-8")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    capture = commands.add_parser("snapshot", help="capture input and generated-file evidence")
    capture.add_argument("--spec", type=Path, required=True)
    capture.add_argument("--generated-root", type=Path, required=True)
    for flag in ("source-url", "generator-version", "revision"):
        capture.add_argument("--" + flag, required=True)
    capture.add_argument("--output", type=Path, required=True)
    diff = commands.add_parser("compare", help="compare snapshots without running the generator")
    diff.add_argument("--before", type=Path, required=True)
    diff.add_argument("--after", type=Path, required=True)
    diff.add_argument("--json-output", type=Path, required=True)
    diff.add_argument("--markdown-output", type=Path, required=True)
    args = parser.parse_args()
    try:
        if args.command == "snapshot":
            output = args.output.resolve()
            if output == args.spec.resolve() or output.is_relative_to(args.generated_root.resolve() / "src"):
                raise ValueError("snapshot output must not overwrite input or generated source")
            write_json(args.output, snapshot(args.spec, args.generated_root, args.source_url,
                                            args.generator_version, args.revision))
        else:
            outputs = {args.json_output.resolve(), args.markdown_output.resolve()}
            if len(outputs) != 2 or outputs & {args.before.resolve(), args.after.resolve()}:
                raise ValueError("report output paths must be distinct from each other and snapshots")
            report = compare(json.loads(args.before.read_text()), json.loads(args.after.read_text()))
            write_json(args.json_output, report)
            args.markdown_output.write_text(markdown(report), encoding="utf-8")
    except (OSError, ValueError, KeyError, TypeError, yaml.YAMLError) as exc:
        parser.exit(2, f"Cannot produce API change report: {exc}\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
