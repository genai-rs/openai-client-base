#!/usr/bin/env python3
"""
Scan generated Rust model files and fix constructor signatures and bodies
for fields typed as Option<Box<T>> so that:

- The corresponding new() parameter uses Option<T>
- The constructor body assigns using param.map(Box::new)

This is needed because OpenAPI Generator often marks fields as optional
without marking them nullable in the spec, so post-generation constructors
don't match the struct field types.
"""

import re
from pathlib import Path
import sys


def find_option_box_fields(content: str):
    """Return list of (field_name, inner_type) for fields Option<Box<inner_type>>."""
    results = []
    # Matches like: pub field_name: Option<Box<models::Type>>,
    field_re = re.compile(r"pub\s+(\w+)\s*:\s*Option\s*<\s*Box\s*<\s*([^>]+)\s*>\s*>\s*,")
    for m in field_re.finditer(content):
        field = m.group(1)
        inner = m.group(2).strip()
        results.append((field, inner))
    return results


def fix_signature(content: str, fields):
    """Update new() parameter types to Option<inner> for the given fields."""
    if not fields:
        return content
    # Find new() signature block
    sig_re = re.compile(r"pub\s+fn\s+new\(([^)]*)\)")
    m = sig_re.search(content)
    if not m:
        return content
    sig = m.group(1)
    orig_sig = sig
    for field, inner in fields:
        # Replace occurrences of 'field: TYPE' to 'field: Option<INNER>' if not already Option<>
        param_re = re.compile(rf"(\b{field}\s*:\s*)(?!Option<)([^,\)]+)")
        sig = param_re.sub(rf"\1Option<{inner}>", sig)
    if sig != orig_sig:
        content = content[: m.start(1)] + sig + content[m.end(1) :]
    return content


def fix_constructor_body(content: str, fields):
    """Update struct literal assignments to use param.map(Box::new)."""
    for field, _inner in fields:
        # Replace field: Box::new(field) => field: field.map(Box::new)
        content = re.sub(
            rf"(\n\s*{field}\s*:\s*)Box::new\({field}\)",
            rf"\1{field}.map(Box::new)",
            content,
        )
        # Replace field: Some(Box::new(field)) => field: field.map(Box::new)
        content = re.sub(
            rf"(\n\s*{field}\s*:\s*)Some\(Box::new\({field}\)\)",
            rf"\1{field}.map(Box::new)",
            content,
        )
    return content


def process_file(path: Path) -> bool:
    txt = path.read_text()
    fields = find_option_box_fields(txt)
    if not fields:
        return False
    new_txt = fix_signature(txt, fields)
    new_txt = fix_constructor_body(new_txt, fields)
    if new_txt != txt:
        path.write_text(new_txt)
        return True
    return False


def main():
    if len(sys.argv) < 2:
        print("Usage: fix_option_box_constructor_mismatches.py <project_root>")
        sys.exit(1)
    root = Path(sys.argv[1])
    models_dir = root / "src" / "models"
    if not models_dir.exists():
        print(f"Models directory not found: {models_dir}")
        sys.exit(1)
    fixed = 0
    for path in models_dir.glob("*.rs"):
        if process_file(path):
            fixed += 1
            print(f"Fixed constructor in {path.name}")
    print(f"Fixed {fixed} files with Option<Box<_>> constructor mismatches")


if __name__ == "__main__":
    main()

