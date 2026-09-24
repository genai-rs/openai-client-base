#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
SPEC_URL="https://raw.githubusercontent.com/openai/openai-openapi/main/openapi.yaml"
SPEC_PATH="$PROJECT_ROOT/stainless.yaml"
TEMP_PATH="$(mktemp "$PROJECT_ROOT/.stainless.yaml.XXXXXX")"
trap 'rm -f "$TEMP_PATH"' EXIT

echo "📥 Fetching the current OpenAI OpenAPI specification from $SPEC_URL"
curl --fail --location --silent --show-error --retry 3 \
    --output "$TEMP_PATH" "$SPEC_URL"

# Validate before replacing the checked-in copy. An HTTP error or malformed
# response must never reach the generator as a null OpenAPI document.
uv run --with pyyaml python "$SCRIPT_DIR/validate_spec.py" "$TEMP_PATH"
mv "$TEMP_PATH" "$SPEC_PATH"
echo "✅ Downloaded OpenAI OpenAPI specification to stainless.yaml"
