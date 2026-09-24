#!/usr/bin/env python3
"""Normalize Rust syntax emitted for valid upstream OpenAPI constructs."""

import re
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
MODELS = ROOT / "src" / "models"


def normalized_name(name: str) -> str:
    # OpenAPI Generator occasionally preserves leading or interior underscores
    # in a Rust type name while mod.rs exports the corresponding PascalCase name.
    return "".join(part[:1].upper() + part[1:] for part in name.strip("_").split("_"))


def normalize_identifiers(files: dict[Path, str]) -> dict[Path, str]:
    names = set()
    for content in files.values():
        names.update(re.findall(r"^pub enum (\w+)\s*\{", content, re.MULTILINE))
    replacements = {
        name: normalized_name(name)
        for name in names
        if name != normalized_name(name)
    }
    if not replacements:
        return files
    distinct = {old: new for old, new in replacements.items() if "_" in old}
    pattern = re.compile(r"\b(" + "|".join(map(re.escape, sorted(distinct, key=len, reverse=True))) + r")\b") if distinct else None
    normalized = {}
    for path, content in files.items():
        if pattern:
            content = pattern.sub(lambda match: distinct[match.group()], content)
        for old, new in replacements.items():
            if "_" not in old:
                # A lowercase schema name can also be a Rust field/module name.
                # Change only the owning enum file and qualified type references.
                if path.stem == old:
                    content = re.sub(rf"\b{re.escape(old)}\b", new, content)
                else:
                    content = content.replace(f"models::{old}", f"models::{new}")
        normalized[path] = content
    return normalized


def fix_content(content: str) -> str:
    lines = content.splitlines(keepends=True)
    result = []
    previous_rename = False
    for line in lines:
        stripped = line.strip()
        is_rename = stripped.startswith("#[serde(rename = ")
        if is_rename and previous_rename:
            # The first rename is the wire value from upstream; a later pass can
            # add a second, conflicting rename for the Rust identifier.
            continue
        is_variant = bool(re.fullmatch(r"\w+\([^)]*\),", stripped))
        is_display_arm = "::" in stripped and "=> write!(f," in stripped
        if result and line == result[-1] and (is_variant or is_display_arm):
            continue
        result.append(line)
        previous_rename = is_rename

    content = "".join(result)
    # The generator gives a non-existent synthetic model name to an unconstrained
    # anyOf branch ({} | null). serde_json::Value represents the upstream value.
    if not (MODELS / "any_of_less_than_greater_than.rs").exists():
        content = content.replace("models::AnyOfLessThanGreaterThan", "serde_json::Value")
    return content


def main() -> None:
    files = {path: path.read_text() for path in MODELS.glob("*.rs")}
    normalized = normalize_identifiers(files)
    changed = 0
    for path, content in normalized.items():
        fixed = fix_content(content)
        if fixed != files[path]:
            path.write_text(fixed)
            changed += 1
    print(f"Normalized generator artifacts in {changed} model files")


if __name__ == "__main__":
    main()
