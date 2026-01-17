#!/usr/bin/env python3
"""
Backfills missing schema definitions in the patched OpenAPI spec.

Some upstream specs reference schemas that are no longer shipped in the
document. This script injects a curated set of fallback schemas so the
generator can resolve all $refs.
"""

import sys
from pathlib import Path

import yaml


def main() -> int:
    if len(sys.argv) < 3:
        print(
            "Usage: patch_missing_schemas.py <spec_path> <overrides_path>",
            file=sys.stderr,
        )
        return 1

    spec_path = Path(sys.argv[1])
    overrides_path = Path(sys.argv[2])

    if not spec_path.exists():
        print(f"Spec file not found: {spec_path}", file=sys.stderr)
        return 1
    if not overrides_path.exists():
        print(f"Overrides file not found: {overrides_path}", file=sys.stderr)
        return 1

    with spec_path.open() as f:
        spec = yaml.safe_load(f)
    with overrides_path.open() as f:
        overrides = yaml.safe_load(f) or {}

    fallback_schemas = overrides.get("schemas") or {}
    if not fallback_schemas:
        print("No fallback schemas provided; nothing to do.")
        return 0

    components = spec.setdefault("components", {})
    schemas = components.setdefault("schemas", {})

    added = []
    for name, definition in fallback_schemas.items():
        if name not in schemas:
            schemas[name] = definition
            added.append(name)

    if added:
        with spec_path.open("w") as f:
            yaml.safe_dump(spec, f, sort_keys=False)
        added_list = ", ".join(sorted(added))
        print(f"Added {len(added)} missing schemas: {added_list}")
    else:
        print("No missing schemas to add.")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
