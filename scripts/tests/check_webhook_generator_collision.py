#!/usr/bin/env python3
"""Exercise the webhook module collision with the pinned Rust generator."""

from __future__ import annotations

import json
import os
import re
import subprocess
import tempfile
from pathlib import Path

import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from route_webhook_apis import route_webhooks


def run_generator(image: str, mounted_dir: Path, input_name: str, output_name: str) -> None:
    subprocess.run(
        [
            "docker", "run", "--rm",
            "-v", f"{mounted_dir}:/local",
            "-u", f"{os.getuid()}:{os.getgid()}",
            "-e", "JAVA_OPTS=-Dlog.level=error",
            image, "generate",
            "-i", f"/local/{input_name}",
            "-g", "rust",
            "-o", f"/local/{output_name}",
            "--skip-validate-spec",
            "--additional-properties=packageName=openai-client-base",
            "--additional-properties=packageVersion=0.1.0",
            "--additional-properties=library=reqwest",
            "--additional-properties=supportAsync=true",
            "--additional-properties=supportMiddleware=true",
            "--additional-properties=preferUnsignedInt=false",
            "--additional-properties=useSingleRequestParameter=false",
        ],
        check=True,
        timeout=180,
    )


def operation_source(path: Path, function: str) -> str:
    source = path.read_text(encoding="utf-8")
    match = re.search(rf"(?m)^pub async fn {re.escape(function)}\(", source)
    if not match:
        raise AssertionError(f"{function} was not generated in {path}")
    return source[match.start():]


def main() -> None:
    version = os.environ.get("OPENAPI_GENERATOR_VERSION", "7.15.0")
    image = f"openapitools/openapi-generator-cli:v{version}"
    fixture = Path(__file__).parent / "fixtures/webhook_collision/openapi.json"
    original = json.loads(fixture.read_text(encoding="utf-8"))
    routed = json.loads(json.dumps(original))
    route_webhooks(routed)

    with tempfile.TemporaryDirectory(prefix="webhook-generator-collision-") as temp:
        root = Path(temp)
        (root / "original.json").write_text(json.dumps(original), encoding="utf-8")
        (root / "routed.json").write_text(json.dumps(routed), encoding="utf-8")
        run_generator(image, root, "original.json", "original")
        run_generator(image, root, "routed.json", "routed")

        original_apis = root / "original/src/apis"
        routed_apis = root / "routed/src/apis"
        if "event_received" not in (original_apis / "default_api.rs").read_text():
            raise AssertionError("baseline did not place the webhook in default_api.rs")
        if "read_untagged" in (original_apis / "default_api.rs").read_text():
            raise AssertionError("baseline fixture no longer reproduces the overwrite")
        if "read_untagged" not in (routed_apis / "default_api.rs").read_text():
            raise AssertionError("routed path operation is missing from default_api.rs")
        if "read_tagged" not in (routed_apis / "probe_api.rs").read_text():
            raise AssertionError("tagged path operation changed during routing")

        webhook_file = routed_apis / "open_ai_client_base_webhooks_api.rs"
        webhook_fn = operation_source(webhook_file, "event_received")
        baseline_fn = operation_source(original_apis / "default_api.rs", "event_received")
        if webhook_fn != baseline_fn:
            raise AssertionError("webhook operation semantics changed when routed")
        if "models::WebhookEvent" not in webhook_fn or "reqwest::Method::POST" not in webhook_fn:
            raise AssertionError("typed webhook request signature or POST method changed")
        for relative in ("models/webhook_event.rs", "models/mod.rs"):
            if (root / "original/src" / relative).read_bytes() != (root / "routed/src" / relative).read_bytes():
                raise AssertionError(f"webhook model changed after routing: {relative}")

    print(f"Pinned generator {version}: collision reproduced and routed webhook semantics preserved.")


if __name__ == "__main__":
    main()
