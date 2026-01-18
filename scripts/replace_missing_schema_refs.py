#!/usr/bin/env python3
"""
Replace references to missing schemas with a free-form object placeholder.

This avoids inventing local schema definitions while still letting generation
proceed when the upstream spec references removed components.
"""

import re
import sys
from pathlib import Path
from typing import Any, Set

import yaml

REF_PATTERN = re.compile(r"#/components/schemas/(?P<name>.+)")


def load_yaml(path: Path) -> Any:
    with path.open() as f:
        return yaml.safe_load(f)


def save_yaml(path: Path, data: Any) -> None:
    with path.open("w") as f:
        yaml.safe_dump(data, f, sort_keys=False)


def collect_defined_schemas(spec: Any) -> Set[str]:
    return set((spec or {}).get("components", {}).get("schemas", {}).keys())


def patch_refs(node: Any, defined: Set[str], missing: Set[str]) -> Any:
    if isinstance(node, dict):
        # If this node is a $ref, check if it's missing
        if "$ref" in node and isinstance(node["$ref"], str):
            m = REF_PATTERN.fullmatch(node["$ref"])
            if m:
                name = m.group("name")
                if name not in defined:
                    missing.add(name)
                    return {
                        "type": "object",
                        "description": f"Upstream schema '{name}' was referenced but not defined; replaced with free-form object to avoid backfilling local definitions."
                    }
        # Otherwise recurse into values
        return {k: patch_refs(v, defined, missing) for k, v in node.items()}
    if isinstance(node, list):
        return [patch_refs(item, defined, missing) for item in node]
    return node


def main() -> int:
    if len(sys.argv) != 2:
        print("Usage: replace_missing_schema_refs.py <spec_path>", file=sys.stderr)
        return 1

    spec_path = Path(sys.argv[1])
    if not spec_path.exists():
        print(f"Spec not found: {spec_path}", file=sys.stderr)
        return 1

    spec = load_yaml(spec_path)
    defined = collect_defined_schemas(spec)
    missing: Set[str] = set()

    patched = patch_refs(spec, defined, missing)
    if missing:
        save_yaml(spec_path, patched)
        joined = ", ".join(sorted(missing))
        print(f"Replaced references to missing schemas with free-form objects: {joined}")
    else:
        print("No missing schema references found.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
