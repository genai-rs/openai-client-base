#!/usr/bin/env python3
"""
Remove or fix helper impls for enums when they reference nonexistent variants.

This addresses cases where helper constructors/defaults generated for untagged unions
do not match the final enum variants (e.g., due to later transformations), causing
compile errors like `no variant or associated item named ... found`.

Strategy:
- For each enum in a model file, collect its variant identifiers.
- Remove impl blocks (`impl EnumName { ... }`) that construct `Self::<Variant>(...)`
  where `<Variant>` is not defined in the enum.
- Remove `impl Default for EnumName` blocks that reference nonexistent variants.
- Remove `impl From<String>` / `impl From<&str>` blocks that reference nonexistent variants.
"""

import re
from pathlib import Path

ROOT = Path(__file__).parent.parent
MODELS = ROOT / 'src' / 'models'

enum_re = re.compile(r"pub\s+enum\s+(\w+)\s*\{([\s\S]*?)\}")
variant_re = re.compile(r"^\s*([A-Za-z0-9_]+)\s*\(", re.MULTILINE)

def collect_enums(src: str):
    enums = {}
    for m in enum_re.finditer(src):
        name = m.group(1)
        body = m.group(2)
        variants = set(v for v in variant_re.findall(body))
        enums[name] = {
            'span': m.span(),
            'variants': variants,
        }
    return enums

def remove_mismatched_impls(src: str):
    enums = collect_enums(src)
    if not enums:
        return src, False
    changed = False

    # Remove impl blocks that reference nonexistent variants
    def find_and_strip_impls(enum_name: str, variants: set, text: str) -> str:
        nonlocal changed
        def strip_blocks(prefix_regex: str, s: str) -> str:
            out = []
            i = 0
            while i < len(s):
                m = re.search(prefix_regex, s[i:])
                if not m:
                    out.append(s[i:])
                    break
                start = i + m.start()
                out.append(s[i:start])
                # Find matching brace starting at the opening '{' at m.end()-1
                brace_start = i + m.end() - 1
                depth = 0
                j = brace_start
                while j < len(s):
                    if s[j] == '{':
                        depth += 1
                    elif s[j] == '}':
                        depth -= 1
                        if depth == 0:
                            break
                    j += 1
                block = s[start:j+1]
                body = s[brace_start+1:j]
                used = set(re.findall(r"Self::([A-Za-z0-9_]+)\s*\(", body))
                if any(u not in variants for u in used):
                    changed = True
                    # drop block
                else:
                    out.append(block)
                i = j+1
            return ''.join(out)

        # impl EnumName { ... }
        # Special fallback: drop helper impls that mention Self::TextContent or Self::ArrayOfContentParts
        # when those variants are not defined
        def strip_helpers(s: str) -> str:
            out = []
            i = 0
            patt = re.compile(rf"impl\s+{enum_name}\s*\{{")
            while i < len(s):
                m = patt.search(s, i)
                if not m:
                    out.append(s[i:])
                    break
                out.append(s[i:m.start()])
                brace_start = m.end() - 1
                depth = 0
                j = brace_start
                while j < len(s):
                    if s[j] == '{':
                        depth += 1
                    elif s[j] == '}':
                        depth -= 1
                        if depth == 0:
                            break
                    j += 1
                block = s[m.start():j+1]
                body = s[brace_start+1:j]
                if (('Self::TextContent(' in body and 'TextContent' not in variants) or
                    ('Self::ArrayOfContentParts(' in body and 'ArrayOfContentParts' not in variants)):
                    changed = True
                    # drop
                else:
                    out.append(block)
                i = j+1
            return ''.join(out)

        text = strip_helpers(text)
        text = strip_blocks(rf"impl\s+{enum_name}\s*\{{", text)
        # impl Default for EnumName { ... }
        text = strip_blocks(rf"impl\s+Default\s+for\s+{enum_name}\s*\{{", text)
        # impl From<...> for EnumName { ... }
        text = strip_blocks(rf"impl\s+From\s*<[^>]+>\s+for\s+{enum_name}\s*\{{", text)
        return text

    new_src = src
    for enum_name, info in enums.items():
        new_src = find_and_strip_impls(enum_name, info['variants'], new_src)
    # Heuristic cleanup for ChatCompletion*MessageContent helpers that reference string/array variants
    if ('Self::TextContent(' in new_src or 'Self::ArrayOfContentParts(' in new_src):
        # Remove any impl blocks containing those helpers regardless of enum variant set
        def drop_blocks_with(mark: str, s: str) -> str:
            out, i = [], 0
            patt = re.compile(r"impl\s+[A-Za-z0-9_<>:&\s]+\{")
            while i < len(s):
                m = patt.search(s, i)
                if not m:
                    out.append(s[i:])
                    break
                out.append(s[i:m.start()])
                brace_start = m.end() - 1
                depth = 0
                j = brace_start
                while j < len(s):
                    if s[j] == '{':
                        depth += 1
                    elif s[j] == '}':
                        depth -= 1
                        if depth == 0:
                            break
                    j += 1
                block = s[m.start():j+1]
                if mark in block:
                    nonlocal changed
                    changed = True
                else:
                    out.append(block)
                i = j+1
            return ''.join(out)
        new_src = drop_blocks_with('Self::TextContent(', new_src)
        new_src = drop_blocks_with('Self::ArrayOfContentParts(', new_src)

    return new_src, changed

def main():
    fixed = 0
    for path in MODELS.glob('*.rs'):
        src = path.read_text()
        new_src, changed = remove_mismatched_impls(src)
        if changed:
            path.write_text(new_src)
            fixed += 1
            print(f"Removed mismatched helpers in {path.name}")
    print(f"Cleaned {fixed} files")

if __name__ == '__main__':
    main()
