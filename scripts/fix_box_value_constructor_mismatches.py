#!/usr/bin/env python3
"""
Fix constructor bodies that still wrap serde_json::Value in Box::new after a type fallback.

Pattern fixed generically:
- In a struct new() body, replace `field: Box::new(field)` with `field: field` when the
  struct defines `pub field: serde_json::Value,`.
"""

import re
from pathlib import Path
import sys

def collect_value_fields(content: str):
    fields = []
    for m in re.finditer(r"\n\s*pub\s+(\w+)\s*:\s*serde_json::Value\s*,", content):
        fields.append(m.group(1))
    return fields

def fix_constructors(content: str, fields):
    if not fields:
        return content
    for f in fields:
        # Replace literal assignments in struct initializer
        content = re.sub(rf"(\n\s*{f}\s*:\s*)Box::new\({f}\)", rf"\1{f}", content)
    return content

def process(path: Path) -> bool:
    src = path.read_text()
    fields = collect_value_fields(src)
    if not fields:
        return False
    new_src = fix_constructors(src, fields)
    if new_src != src:
        path.write_text(new_src)
        return True
    return False

def main():
    root = Path(__file__).parent.parent
    models = root / 'src' / 'models'
    fixed = 0
    for p in models.glob('*.rs'):
        if process(p):
            fixed += 1
            print(f"Fixed Box<serde_json::Value> constructor mismatch in {p.name}")
    print(f"Fixed {fixed} files")

if __name__ == '__main__':
    main()

