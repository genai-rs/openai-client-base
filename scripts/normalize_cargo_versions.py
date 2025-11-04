#!/usr/bin/env python3
"""
Normalize Cargo dependency versions so generated code keeps caret-prefixed semver ranges.

OpenAPI generation emits exact versions (e.g. `"1.2.3"`), but our crates prefer `^1.2.3`
to allow compatible upgrades. Run this script after generation to rewrite plain numeric
version strings located inside dependency tables.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

TARGET_TABLES = {
    "[dependencies]",
    "[dev-dependencies]",
    "[build-dependencies]",
}

SKIP_PREFIXES = ("^", "~", "<", ">", "=", "*", "!")


def should_upgrade(raw_version: str) -> bool:
    trimmed = raw_version.strip()
    if not trimmed:
        return False
    if trimmed.startswith(SKIP_PREFIXES):
        return False
    return bool(re.match(r"\d", trimmed))


def transform_line(line: str) -> str:

    def replace_simple(match: re.Match[str]) -> str:
        prefix, version = match.groups()
        if should_upgrade(version):
            return f'{prefix}"^{version}"'
        return match.group(0)

    def replace_table(match: re.Match[str]) -> str:
        prefix, version = match.groups()
        if should_upgrade(version):
            return f'{prefix}"^{version}"'
        return match.group(0)

    simple_dep_re = re.compile(
        r'^(\s*[A-Za-z0-9_\-]+(?:\s*\.\s*[A-Za-z0-9_\-]+)?\s*=\s*)"([^"]+)"'
    )
    version_field_re = re.compile(r'(version\s*=\s*)"([^"]+)"')

    updated = simple_dep_re.sub(replace_simple, line)
    updated = version_field_re.sub(replace_table, updated)
    return updated


def normalize_cargo(file_path: Path) -> None:
    original = file_path.read_text()
    lines = original.splitlines()

    normalized_lines = []
    in_target_table = False

    for line in lines:
        stripped = line.strip()
        if stripped.startswith("[") and stripped.endswith("]"):
            in_target_table = stripped in TARGET_TABLES

        if in_target_table:
            normalized_lines.append(transform_line(line))
        else:
            normalized_lines.append(line)

    trailing_newline = "\n" if original.endswith("\n") else ""
    file_path.write_text("\n".join(normalized_lines) + trailing_newline)


def main(argv: list[str]) -> int:
    if len(argv) != 2:
        print("Usage: normalize_cargo_versions.py <path-to-Cargo.toml>", file=sys.stderr)
        return 1

    cargo_path = Path(argv[1]).resolve()
    if not cargo_path.exists():
        print(f"Cargo.toml not found: {cargo_path}", file=sys.stderr)
        return 1

    normalize_cargo(cargo_path)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
