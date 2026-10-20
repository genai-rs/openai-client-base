#!/usr/bin/env python3
"""Keep legacy default_api webhook paths after routing webhooks to their own module."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


def preserve_webhook_api(apis_dir: Path, webhook_module: str = "open_ai_client_base_webhooks_api") -> None:
    """Re-export routed webhook operations from the legacy default_api module.

    Before routing, webhook functions were emitted in default_api.rs. The
    re-export retains their existing public paths while normal untagged path
    operations are generated there. Create the module when no path operation
    happens to use the default tag.
    """
    module_file = apis_dir / "mod.rs"
    routed_file = apis_dir / f"{webhook_module}.rs"
    if not routed_file.is_file() or not module_file.is_file():
        raise FileNotFoundError(
            f"Expected routed webhook module and API index: {routed_file}, {module_file}"
        )

    module_source = module_file.read_text(encoding="utf-8")
    module_source = ensure_module_decl(module_source, webhook_module)
    module_source = ensure_module_decl(module_source, "default_api")
    default_file = apis_dir / "default_api.rs"
    source = (
        default_file.read_text(encoding="utf-8")
        if default_file.exists()
        else "// Compatibility exports for webhook operations formerly generated here.\n"
    )
    reexport = f"pub use super::{webhook_module}::*;"
    reexports = len(re.findall(rf"(?m)^\s*{re.escape(reexport)}\s*$", source))
    if reexports > 1:
        raise ValueError(f"default_api contains duplicate webhook re-exports for {webhook_module!r}")
    if reexports == 0:
        source = source.rstrip() + "\n\n" + reexport + "\n"

    # Complete validation before changing either file.
    if module_source != module_file.read_text(encoding="utf-8"):
        module_file.write_text(module_source, encoding="utf-8")
    if not default_file.exists() or source != default_file.read_text(encoding="utf-8"):
        default_file.write_text(source, encoding="utf-8")


def ensure_module_decl(source: str, module_name: str) -> str:
    """Return source with exactly one public module declaration."""
    declaration = re.compile(
        rf"(?m)^[ \t]*(?P<visibility>pub(?:\([^)]*\))?[ \t]+)?mod[ \t]+{re.escape(module_name)};[ \t]*$"
    )
    matches = list(declaration.finditer(source))
    if len(matches) > 1:
        raise ValueError(
            f"API index contains duplicate declarations ({len(matches)}) for module {module_name!r}"
        )
    if len(matches) == 1:
        if matches[0].group("visibility") not in {"pub ", "pub\t"}:
            raise ValueError(f"API index has a non-public declaration for module {module_name!r}")
        return source
    return source.rstrip() + f"\n\npub mod {module_name};\n"


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("apis_dir", type=Path)
    parser.add_argument("--webhook-module", default="open_ai_client_base_webhooks_api")
    args = parser.parse_args()
    preserve_webhook_api(args.apis_dir, args.webhook_module)


if __name__ == "__main__":
    main()
