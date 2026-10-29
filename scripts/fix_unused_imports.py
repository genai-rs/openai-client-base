#!/usr/bin/env python3
"""
Remove imports the generator emits unconditionally but the file never uses.

Replaces the crate-wide `#![allow(unused_imports)]`. Each rule only drops an
import when its bound name does not appear anywhere else in the file, so the
script is idempotent and safe to run on already-clean files.
"""

import re
import sys
from pathlib import Path


def code_only(content: str) -> str:
    """Drop comment lines (including doc comments); mentions there are not uses."""
    return "\n".join(l for l in content.split("\n") if not l.lstrip().startswith("//"))


def used_elsewhere(content: str, removed: str, name: str) -> bool:
    code = code_only(content.replace(removed, "", 1))
    return re.search(r"\b" + re.escape(name) + r"\b", code) is not None


def drop_line(content: str, line: str, names: list[str]) -> str:
    if line not in content:
        return content
    if any(used_elsewhere(content, line, n) for n in names):
        return content
    return content.replace(line + "\n", "", 1)


def prune_braced(content: str, prefix: str, items: list[str]) -> str:
    """Drop unused names from `use <prefix>::{a, b, c};` / `use <prefix>::a;`."""
    pattern = re.compile(r"^use " + re.escape(prefix) + r"::\{([^}]*)\};\n", re.M)
    match = pattern.search(content)
    if not match:
        return content
    stmt = match.group(0)
    rest = code_only(content.replace(stmt, "", 1))
    kept = [
        i.strip()
        for i in match.group(1).split(",")
        if i.strip()
        and (i.strip() not in items or re.search(r"\b" + re.escape(i.strip()) + r"\b", rest))
    ]
    if kept == [i.strip() for i in match.group(1).split(",") if i.strip()]:
        return content
    new = f"use {prefix}::{{{', '.join(kept)}}};\n" if kept else ""
    return content.replace(stmt, new, 1)


def fix_file(path: Path) -> bool:
    original = path.read_text()
    content = original

    # Models (and API files) that never reference `models::`.
    if re.search(r"^use crate::models;\n", content, re.M):
        content = drop_line(content, "use crate::models;", ["models"])

    # Multipart upload helpers imported into every API module.
    content = drop_line(content, "use tokio::fs::File as TokioFile;", ["TokioFile"])
    content = prune_braced(content, "tokio_util::codec", ["BytesCodec", "FramedRead"])
    if re.search(r"^use tokio_util::codec::\{\s*\};", content, re.M):
        content = re.sub(r"^use tokio_util::codec::\{\s*\};\n", "", content, flags=re.M)
    content = prune_braced(content, "super", ["ContentType", "multipart_helper"])

    # Trait import only needed for `Error::custom`-style calls. The generator writes the
    # names in its own order (`{Deserialize, Serialize, de::Error as _}`); cargo fmt only
    # sorts them later, so match the statement whatever the order.
    serde_use = re.search(r"^use serde::\{([^}]*\bde::Error as _\b[^}]*)\};\n", content, re.M)
    if serde_use:
        stripped = content.replace(serde_use.group(0), "", 1)
        if not re.search(r"\.custom\(|::custom\(|\.invalid_|\.missing_field\(|\.unknown_variant\(", stripped):
            kept = [i.strip() for i in serde_use.group(1).split(",") if i.strip() and i.strip() != "de::Error as _"]
            content = content.replace(serde_use.group(0), f"use serde::{{{', '.join(kept)}}};\n", 1)

    if content != original:
        path.write_text(content)
        return True
    return False


def main() -> None:
    root = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).parent.parent
    changed = sum(fix_file(p) for sub in ("apis", "models") for p in sorted((root / "src" / sub).glob("*.rs")))
    print(f"Removed unused imports from {changed} files")


if __name__ == "__main__":
    main()
