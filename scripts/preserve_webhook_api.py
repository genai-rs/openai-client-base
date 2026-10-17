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

    default_file = apis_dir / "default_api.rs"
    if not default_file.exists():
        default_file.write_text(
            "// Compatibility exports for webhook operations formerly generated here.\n"
            f"pub use super::{webhook_module}::*;\n",
            encoding="utf-8",
        )
        if not re.search(r"(?m)^pub mod default_api;\s*$", module_file.read_text()):
            with module_file.open("a", encoding="utf-8") as stream:
                stream.write("\npub mod default_api;\n")

    source = default_file.read_text(encoding="utf-8")
    reexport = f"pub use super::{webhook_module}::*;"
    if reexport not in source:
        default_file.write_text(source.rstrip() + "\n\n" + reexport + "\n", encoding="utf-8")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("apis_dir", type=Path)
    parser.add_argument("--webhook-module", default="open_ai_client_base_webhooks_api")
    args = parser.parse_args()
    preserve_webhook_api(args.apis_dir, args.webhook_module)


if __name__ == "__main__":
    main()
