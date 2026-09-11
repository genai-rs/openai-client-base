#!/usr/bin/env python3
"""Keep generated API errors small regardless of the response schema's size.

Patch the generator's shared transport type and its constructor expressions,
without changing any model, serialization, or endpoint signature. Fail if a new
generator changes the expected shape rather than silently leaving large errors.
"""

import re
import sys
from pathlib import Path


def patch_module(source: str) -> str:
    pattern = r"\bResponseError\(\s*ResponseContent<T>\s*\)"
    boxed = "ResponseError(Box<ResponseContent<T>>)"
    if boxed in source:
        return source
    patched, count = re.subn(pattern, boxed, source)
    if count != 1:
        raise ValueError("Expected one shared ResponseError(ResponseContent<T>) variant")
    return patched


def patch_constructors(source: str, *, allow_patterns: bool = False) -> str:
    # Generated response literals contain status, content and entity fields.
    # Restrict matching to constructor expressions, leaving patterns untouched.
    pattern = r"(\bError::ResponseError\(\s*)(ResponseContent\s*\{[^{}]*\})(\s*\))"
    patched = re.sub(pattern, r"\1Box::new(\2)\3", source)
    if re.search(r"\bError::ResponseError\(\s*ResponseContent\b", patched):
        raise ValueError("Unsupported generated ResponseError constructor")
    # Only the shared module contains match patterns. Endpoint modules must
    # construct boxed payloads, including if a future template uses a variable.
    if not allow_patterns and re.search(
        r"\bError::ResponseError\((?!\s*Box::new\s*\()", patched
    ):
        raise ValueError("Unsupported generated ResponseError constructor")
    return patched


def patch_directory(apis: Path) -> None:
    module = apis / "mod.rs"
    # Validate all transformations before writing any files.
    updates = {module: patch_module(module.read_text())}
    for path in sorted(apis.glob("*.rs")):
        source = updates.get(path, path.read_text())
        updates[path] = patch_constructors(source, allow_patterns=path == module)
    for path, source in updates.items():
        if source != path.read_text():
            path.write_text(source)


if __name__ == "__main__":
    root = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent
    patch_directory(root / "src" / "apis")
