#!/usr/bin/env python3
"""Route OpenAPI webhooks into a dedicated Rust API module.

OpenAPI Generator's Rust generator writes path operations without tags and
webhooks into ``default_api.rs``. A webhook written second replaces those path
operations. This adds a generator-only tag to webhook operation metadata. It
does not remove webhooks or change their schemas, methods, paths, or payloads.
The unmodified source bytes remain at the input path.
"""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

import yaml

WEBHOOK_API_TAG = "OpenAiClientBaseWebhooks"
HTTP_METHODS = {"get", "put", "post", "delete", "options", "head", "patch", "trace"}


def route_webhooks(spec: dict[str, Any]) -> int:
    paths = spec.get("paths", {})
    for path, item in paths.items():
        if not isinstance(item, dict):
            continue
        for method, operation in item.items():
            if method in HTTP_METHODS and isinstance(operation, dict):
                if WEBHOOK_API_TAG in operation.get("tags", []):
                    raise ValueError(
                        f"Reserved webhook routing tag {WEBHOOK_API_TAG!r} is already "
                        f"used by normal path operation {method.upper()} {path}"
                    )

    changed = 0
    webhooks = spec.get("webhooks", {})
    if not isinstance(webhooks, dict):
        raise ValueError("OpenAPI webhooks must be an object")
    for name, item in webhooks.items():
        if not isinstance(item, dict):
            raise ValueError(f"Webhook {name!r} must be an object")
        for method, operation in item.items():
            if method not in HTTP_METHODS:
                continue
            if not isinstance(operation, dict):
                raise ValueError(f"Webhook {name!r} {method.upper()} must be an object")
            tags = operation.setdefault("tags", [])
            if not isinstance(tags, list) or not all(isinstance(tag, str) for tag in tags):
                raise ValueError(f"Webhook {name!r} {method.upper()} tags must be strings")
            if WEBHOOK_API_TAG not in tags:
                tags.append(WEBHOOK_API_TAG)
                changed += 1
    return changed


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("input", type=Path)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()

    spec = yaml.safe_load(args.input.read_text(encoding="utf-8"))
    if not isinstance(spec, dict):
        raise ValueError("OpenAPI document must be an object")
    count = route_webhooks(spec)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(yaml.safe_dump(spec, sort_keys=False), encoding="utf-8")
    print(f"Routed {count} webhook operation(s) to API tag {WEBHOOK_API_TAG}.")


if __name__ == "__main__":
    main()
