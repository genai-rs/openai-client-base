#!/usr/bin/env python3
"""
Generic fixer for enum attributes in generated Rust models.

Problems addressed generically:
- Derive attribute text accidentally concatenated into a preceding doc comment line.
- Enums missing Serialize/Deserialize derives when they have serde attributes (tagged or untagged).

This script scans all files under src/models and fixes patterns without targeting
any specific enum by name.
"""

from pathlib import Path
import re

ROOT = Path(__file__).parent.parent
MODELS = ROOT / 'src' / 'models'

def split_doc_and_attribute(content: str) -> str:
    # If a doc comment line contains a derive attribute (e.g. due to missing newline),
    # split it so that #[derive(...)] starts on its own line.
    pattern = re.compile(r'^(?P<prefix>\s*///.*?)(?P<attr>\#\[derive\([^\)]*\)\])', re.MULTILINE)
    def repl(m):
        return f"{m.group('prefix')}\n{m.group('attr')}"
    return re.sub(pattern, repl, content)

def ensure_enum_derives(content: str) -> str:
    lines = content.split('\n')
    out = []
    i = 0
    changed = False
    while i < len(lines):
        out.append(lines[i])
        # Detect serde attribute on next line followed by pub enum
        if re.match(r'^\s*#\[serde\(', lines[i]):
            # Look ahead to find the enum line (skip blank lines)
            j = i + 1
            while j < len(lines) and lines[j].strip() == '':
                out.append(lines[j])
                j += 1
            if j < len(lines) and re.match(r'^\s*pub\s+enum\s+\w+', lines[j]):
                # Check previous non-empty line for an existing derive
                k = len(out) - 2
                has_derive = False
                while k >= 0 and out[k].strip() == '':
                    k -= 1
                if k >= 0 and out[k].lstrip().startswith('#[derive('):
                    has_derive = True
                if not has_derive:
                    out.insert(k + 1, '#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]')
                    changed = True
        i += 1
    return ('\n'.join(out), changed)

def main():
    if not MODELS.exists():
        return
    total_changes = 0
    for path in MODELS.glob('*.rs'):
        src = path.read_text()
        original = src
        src = split_doc_and_attribute(src)
        src2, changed = ensure_enum_derives(src)
        if src2 != original:
            path.write_text(src2)
            total_changes += 1
    print(f"Fixed enum attributes in {total_changes} files")

if __name__ == '__main__':
    main()

