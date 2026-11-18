#!/usr/bin/env python3
"""Write the generator configuration of openai-server-base from the client's (scripts/generator-config.json).

The server must be generated with the same model options as openai-client-base, whose models it uses
(timvw-rust's modelsCrate): it keeps every option of the client's configuration except the client-only ones, and
adds the server's. Usage: server_config.py <generator-config.json> <output.json>
"""

import json
import sys

# Options timvw-rust applies to the client only: a server-only generation fails on them. (The query-style
# options, deepObjectQueryParameters and resetQueryParameterStyle, apply to both: client and server must agree.)
CLIENT_ONLY = {
    "runtimeVersion",
    "runtimePath",
    "reqwestDefaultFeatures",
    "responseFunctions",
    "supportMultipleResponses",
    "useSerdePathToError",
    "contentTypeMethods",
    "typedEventStreams",
    "typedEnumParameters",
    "multipartHelper",
    "userAgentFromPackage",
    "apiLintAllows",
    "webhookModuleReexport",
}

# Debugging output of the client's generation: the server's would overwrite it.
DROPPED = {"preprocessedSpecFile", "unionPlanFile"}

SERVER = {
    "packageName": "openai-server-base",
    "targets": ["server"],
    "modelsCrate": "openai-client-base",
    "modelsCratePath": "..",
    "serverDefaultMethods": True,
}


def server_config(client: dict) -> dict:
    config = {k: v for k, v in client.items() if k not in CLIENT_ONLY and k not in DROPPED}
    config.update(SERVER)
    return config


def main(argv: list[str]) -> int:
    if len(argv) != 3:
        print(__doc__.strip().splitlines()[-1], file=sys.stderr)
        return 2
    with open(argv[1], encoding="utf-8") as f:
        client = json.load(f)
    with open(argv[2], "w", encoding="utf-8") as f:
        json.dump(server_config(client), f, indent=2)
        f.write("\n")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
