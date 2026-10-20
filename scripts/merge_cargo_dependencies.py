#!/usr/bin/env python3
"""
Merge dependency tables from the OpenAPI generator without overwriting curated versions.

The generator rewrites `Cargo.toml` with exact dependency versions. We prefer to keep the
versions we curate in the repository so regeneration does not churn dependency updates.
This script merges the generated dependency tables into the existing `Cargo.toml` while
preserving the version requirement (including any leading range operators) for entries
that already exist in the project.

New dependencies from the generated file are appended as-is so the build continues to
work if upstream templates start requiring additional crates.
"""

from __future__ import annotations

import re
import sys
import json
import tomllib
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, List, Optional, Tuple

SECTION_NAMES = ("dependencies", "dev-dependencies", "build-dependencies")


@dataclass
class Entry:
    key: Optional[str]
    text: str


def extract_section(body: str, name: str) -> Tuple[List[Entry], Dict[str, Entry]]:
    """Return the parsed entries and mapping for the given dependency section."""
    pattern = re.compile(
        rf"(?ms)^\[{re.escape(name)}\]\n(.*?)(?=^\[|\Z)"
    )
    match = pattern.search(body)
    if not match:
        return [], {}

    section_body = match.group(1)
    entries: List[Entry] = []
    mapping: Dict[str, Entry] = {}

    current_key: Optional[str] = None
    current_lines: List[str] = []

    def flush() -> None:
        nonlocal current_key, current_lines
        if current_lines:
            entry_text = "\n".join(current_lines)
            entry = Entry(current_key, entry_text)
            entries.append(entry)
            if current_key is not None:
                mapping.setdefault(current_key, entry)
        current_key = None
        current_lines = []

    for raw_line in section_body.splitlines():
        line = raw_line.rstrip("\n")
        stripped = line.strip()

        if not stripped:
            flush()
            entries.append(Entry(None, line))
            continue

        if stripped.startswith("#"):
            flush()
            entries.append(Entry(None, line))
            continue

        key_match = re.match(r"\s*([A-Za-z0-9_\-]+)\s*=", line)
        if key_match:
            flush()
            current_key = key_match.group(1)
            current_lines = [line]
        else:
            if not current_lines:
                entries.append(Entry(None, line))
            else:
                current_lines.append(line)

    flush()
    return entries, mapping


def extract_version(entry_text: str) -> Optional[str]:
    """Return the version string from a dependency entry if present."""
    simple_match = re.match(
        r'\s*[A-Za-z0-9_\-]+\s*=\s*"([^"]+)"', entry_text, flags=re.DOTALL
    )
    if simple_match:
        return simple_match.group(1)

    version_match = re.search(r'(version\s*=\s*")([^"]+)(")', entry_text)
    if version_match:
        return version_match.group(2)
    return None


def replace_version(entry_text: str, new_version: str) -> str:
    """Replace the version string inside an entry with the provided value."""
    if "version" in entry_text:
        return re.sub(
            r'(version\s*=\s*")([^"]+)(")',
            lambda m: f'{m.group(1)}{new_version}{m.group(3)}',
            entry_text,
            count=1,
        )

    return re.sub(
        r'^(\s*[A-Za-z0-9_\-]+\s*=\s*")([^"]+)(".*)',
        lambda m: f'{m.group(1)}{new_version}{m.group(3)}',
        entry_text,
        count=1,
        flags=re.DOTALL,
    )


def render_section(header: str, entries: List[Entry]) -> str:
    lines: List[str] = [f"[{header}]"]
    for entry in entries:
        lines.append(entry.text)
    if lines and lines[-1] != "":
        lines.append("")
    return "\n".join(lines)


def merge_section(
    section: str,
    original_entries: List[Entry],
    generated_entries: List[Entry],
    generated_map: Dict[str, Entry],
    key_to_generated_section: Dict[str, str],
    original_map: Dict[str, Entry],
) -> List[Entry]:
    """Merge dependency entries while preserving curated versions and avoiding duplicates."""
    merged: List[Entry] = []
    processed: set[str] = set()

    for entry in original_entries:
        if entry.key is None:
            merged.append(entry)
            continue

        generated_section = key_to_generated_section.get(entry.key)
        if generated_section is None:
            merged.append(entry)
            continue

        if generated_section != section:
            merged.append(entry)
            continue

        generated_entry = generated_map.get(entry.key)
        if generated_entry is None:
            merged.append(entry)
            continue

        updated_text = merge_dependency_entry(section, entry, generated_entry)

        merged.append(Entry(entry.key, updated_text))
        processed.add(entry.key)

    for entry in generated_entries:
        if entry.key is None:
            continue
        if entry.key in processed:
            continue
        if key_to_generated_section.get(entry.key) != section:
            continue

        updated_text = entry.text
        curated_entry = original_map.get(entry.key)
        if curated_entry is not None:
            updated_text = merge_dependency_entry(section, curated_entry, entry)

        merged.append(Entry(entry.key, updated_text))
        processed.add(entry.key)

    return merged


def parse_dependency_value(section: str, entry: Entry) -> Dict[str, object]:
    """Parse a dependency entry as TOML without losing its dependency key."""
    if entry.key is None:
        raise ValueError("cannot parse an unkeyed dependency entry")
    try:
        value = tomllib.loads(f"[{section}]\n{entry.text}\n")[section][entry.key]
    except (tomllib.TOMLDecodeError, KeyError) as exc:
        raise ValueError(
            f"cannot parse {section}.{entry.key} dependency entry: {exc}"
        ) from exc
    if isinstance(value, str):
        return {"version": value}
    if not isinstance(value, dict):
        raise ValueError(
            f"unsupported {section}.{entry.key} dependency representation: "
            f"{type(value).__name__}"
        )
    return value


def toml_value(value: object) -> str:
    """Render the TOML value types accepted in Cargo dependency inline tables."""
    if isinstance(value, str):
        return json.dumps(value, ensure_ascii=False)
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, (int, float)):
        return str(value)
    if isinstance(value, list):
        return "[" + ", ".join(toml_value(item) for item in value) + "]"
    if isinstance(value, dict):
        return "{ " + ", ".join(
            f"{key} = {toml_value(item)}" for key, item in value.items()
        ) + " }"
    raise ValueError(f"unsupported Cargo dependency TOML value: {value!r}")


def merge_dependency_entry(section: str, curated: Entry, generated: Entry) -> str:
    """Keep curated dependency policy while adding generated feature needs."""
    curated_value = parse_dependency_value(section, curated)
    generated_value = parse_dependency_value(section, generated)

    curated_package = curated_value.get("package", curated.key)
    generated_package = generated_value.get("package", generated.key)
    if curated_package != generated_package:
        raise ValueError(
            f"conflicting package mappings for {section}.{curated.key}: "
            f"curated={curated_package!r}, generated={generated_package!r}"
        )

    source_keys = ("git", "path", "registry", "registry-index")
    source_ref_keys = ("branch", "tag", "rev")
    def effective_source(value: Dict[str, object]) -> tuple[object, ...]:
        source_fields = [
            key for key in ("git", "path", "registry", "registry-index") if key in value
        ]
        if len(source_fields) > 1:
            raise ValueError(
                f"unsupported multiple Cargo source selectors for {section}.{curated.key}: {source_fields}"
            )
        if value.get("workspace") is True:
            if source_fields:
                raise ValueError(
                    f"unsupported workspace and source selectors for {section}.{curated.key}: {source_fields}"
                )
            return ("workspace",)
        if "git" in value:
            selector = next(
                (name for name in ("rev", "tag", "branch") if name in value),
                "default-ref",
            )
            return ("git", value["git"], selector, value.get(selector))
        if "path" in value:
            return ("path", value["path"])
        if "registry-index" in value:
            return ("registry-index", value["registry-index"])
        return ("registry", value.get("registry", "crates-io"))

    curated_source = effective_source(curated_value)
    generated_source = effective_source(generated_value)
    if curated_source != generated_source:
        raise ValueError(
            f"conflicting source mappings for {section}.{curated.key}: "
            f"curated={curated_source!r}, generated={generated_source!r}"
        )

    # Cargo gives absent policy keys effective values. Compare those values,
    # then preserve the curated representation so a generator cannot change
    # implicit required/default-registry/default-feature behavior.
    policy_defaults: Dict[str, object] = {
        "optional": False,
        "default-features": True,
        "workspace": False,
    }
    for policy, default in policy_defaults.items():
        for label, value in (("curated", curated_value), ("generated", generated_value)):
            if policy in value and not isinstance(value[policy], bool):
                raise ValueError(
                    f"{label} {policy} for {section}.{curated.key} must be boolean"
                )
        if curated_value.get(policy, default) != generated_value.get(policy, default):
            raise ValueError(
                f"conflicting effective {policy} policy for {section}.{curated.key}: "
                f"curated={curated_value.get(policy, default)!r}, "
                f"generated={generated_value.get(policy, default)!r}"
            )

    source_and_policy_keys = set(source_keys) | set(source_ref_keys) | set(policy_defaults) | {"package"}
    merged = {
        key: value
        for key, value in generated_value.items()
        if key not in source_and_policy_keys
    }
    # Keep curated keys and their representation, including implicit defaults.
    for key, value in curated_value.items():
        if key == "features":
            continue
        merged[key] = value

    curated_features = curated_value.get("features", [])
    generated_features = generated_value.get("features", [])
    for label, features in (("curated", curated_features), ("generated", generated_features)):
        if not isinstance(features, list) or not all(isinstance(item, str) for item in features):
            raise ValueError(
                f"{label} features for {section}.{curated.key} must be an array of strings"
            )
    merged_features = list(dict.fromkeys([*curated_features, *generated_features]))
    if merged_features or "features" in curated_value or "features" in generated_value:
        merged["features"] = merged_features

    if "version" not in merged and "version" in curated_value:
        merged["version"] = curated_value["version"]
    return f"{curated.key} = {{ " + ", ".join(
        f"{key} = {toml_value(value)}" for key, value in merged.items()
    ) + " }"


def update_cargo(original: Path, generated: Path) -> None:
    original_text = original.read_text()
    generated_text = generated.read_text()

    updated_text = original_text

    original_sections: Dict[str, List[Entry]] = {}
    original_maps: Dict[str, Dict[str, Entry]] = {}
    for section in SECTION_NAMES:
        orig_entries, orig_map = extract_section(updated_text, section)
        original_sections[section] = orig_entries
        original_maps[section] = orig_map

    generated_sections: Dict[str, Tuple[List[Entry], Dict[str, Entry]]] = {}
    key_to_generated_section: Dict[str, str] = {}
    for section in SECTION_NAMES:
        gen_entries, gen_map = extract_section(generated_text, section)
        generated_sections[section] = (gen_entries, gen_map)
        for key in gen_map:
            key_to_generated_section[key] = section

    for section in SECTION_NAMES:
        orig_entries = original_sections.get(section, [])
        gen_entries, gen_map = generated_sections.get(section, ([], {}))

        if not gen_entries and not orig_entries:
            continue

        merged_entries = merge_section(
            section,
            orig_entries,
            gen_entries,
            gen_map,
            key_to_generated_section,
            original_maps.get(section, {}),
        )
        if merged_entries:
            new_section = render_section(section, merged_entries).rstrip("\n") + "\n\n"
        else:
            new_section = ""

        pattern = re.compile(
            rf"(?ms)^\[{re.escape(section)}\]\n.*?(?=^\[|\Z)"
        )

        if pattern.search(updated_text):
            updated_text = pattern.sub(new_section, updated_text, count=1)
        else:
            if new_section:
                if not updated_text.endswith("\n"):
                    updated_text += "\n"
                updated_text += new_section + "\n"

    updated_text = updated_text.rstrip("\n") + "\n"
    original.write_text(updated_text)


def main(argv: List[str]) -> int:
    if len(argv) != 3:
        print(
            "Usage: merge_cargo_dependencies.py <path-to-Cargo.toml> <path-to-generated>",
            file=sys.stderr,
        )
        return 1

    cargo_path = Path(argv[1]).resolve()
    generated_path = Path(argv[2]).resolve()

    if not cargo_path.exists():
        print(f"Cargo.toml not found: {cargo_path}", file=sys.stderr)
        return 1
    if not generated_path.exists():
        print(f"Generated Cargo.toml not found: {generated_path}", file=sys.stderr)
        return 1

    update_cargo(cargo_path, generated_path)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
