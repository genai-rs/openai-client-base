#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "📥 Fetching Stainless OpenAI specification..."


# Use Stainless spec as the single authoritative source
# Stainless is OpenAI's official SDK generation partner
STAINLESS_SPEC_URL="https://app.stainless.com/api/spec/documented/openai/openapi.documented.yml"

# Fetch the spec
echo "Downloading from: $STAINLESS_SPEC_URL"
echo "Note: Stainless spec is the authoritative source (no website cross-referencing needed)"
curl -s -o "$PROJECT_ROOT/stainless.yaml" "$STAINLESS_SPEC_URL"

if [ -f "$PROJECT_ROOT/stainless.yaml" ]; then
    echo "✅ Successfully downloaded Stainless spec to stainless.yaml"
    
    # Get spec version info
    VERSION=$(grep -m1 "version:" "$PROJECT_ROOT/stainless.yaml" | cut -d'"' -f2 || echo "unknown")
    echo "📌 Spec version: $VERSION"
    
else
    echo "❌ Failed to download Stainless spec"
    exit 1
fi