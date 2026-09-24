#!/usr/bin/env python3
"""Reject malformed OpenAPI input and unresolved local references."""

import sys
from pathlib import Path
from typing import Any

import yaml


def validate(spec: Any) -> list[str]:
    errors: list[str] = []
    if not isinstance(spec, dict):
        return ["OpenAPI document must be an object"]
    if not isinstance(spec.get("openapi"), str) or not spec["openapi"].startswith("3."):
        errors.append("missing or unsupported OpenAPI version")
    if not isinstance(spec.get("paths"), dict) or not spec["paths"]:
        errors.append("missing or empty paths")
    components = spec.get("components")
    schemas = components.get("schemas") if isinstance(components, dict) else None
    if not isinstance(schemas, dict) or not schemas:
        errors.append("missing or empty components.schemas")
    if errors:
        return errors

    def walk(node: Any, location: str) -> None:
        if isinstance(node, dict):
            ref = node.get("$ref")
            if isinstance(ref, str) and ref.startswith("#/"):
                target: Any = spec
                for token in ref[2:].split("/"):
                    token = token.replace("~1", "/").replace("~0", "~")
                    if not isinstance(target, dict) or token not in target:
                        errors.append(f"{location}: unresolved $ref {ref}")
                        break
                    target = target[token]
            for key, value in node.items():
                walk(value, f"{location}/{key}")
        elif isinstance(node, list):
            for index, value in enumerate(node):
                walk(value, f"{location}/{index}")

    walk(spec, "#")
    return errors


def main() -> int:
    if len(sys.argv) != 2:
        print("Usage: validate_spec.py <spec_path>", file=sys.stderr)
        return 2
    path = Path(sys.argv[1])
    try:
        with path.open(encoding="utf-8") as source:
            spec = yaml.safe_load(source)
    except (OSError, yaml.YAMLError) as exc:
        print(f"Cannot parse OpenAPI document {path}: {exc}", file=sys.stderr)
        return 1

    errors = validate(spec)
    if errors:
        for error in errors:
            print(f"Invalid OpenAPI document {path}: {error}", file=sys.stderr)
        return 1
    print(f"Validated OpenAPI {spec['openapi']} with {len(spec['paths'])} paths and "
          f"{len(spec['components']['schemas'])} schemas: {path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
