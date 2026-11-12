#!/usr/bin/env bash
set -euo pipefail

# Config
OPENAPI_GENERATOR_VERSION="${OPENAPI_GENERATOR_VERSION:-7.27.0}"
# Custom generator image (timvw-rust is an openapi-generator plugin with its own
# Rust layer and templates; see
# https://github.com/timvw/openapi-generator-timvw-rust). Image tags are
# <generator version>-og<openapi-generator version>. Override for local builds,
# e.g. GENERATOR_IMAGE=timvw-rust:dev.
TIMVW_RUST_VERSION="${TIMVW_RUST_VERSION:-0.25.0}"
GENERATOR_IMAGE="${GENERATOR_IMAGE:-ghcr.io/timvw/openapi-generator-timvw-rust:${TIMVW_RUST_VERSION}-og${OPENAPI_GENERATOR_VERSION}}"
GENERATOR_NAME="${GENERATOR_NAME:-timvw-rust}"
# Control openapi-generator verbosity (values: error|warn|info|debug|trace)
OPENAPI_GENERATOR_LOG_LEVEL="${OPENAPI_GENERATOR_LOG_LEVEL:-info}"
PKG_NAME="openai-client-base"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Avoid writing Python bytecode (.pyc) / __pycache__ during CI/generation
export PYTHONDONTWRITEBYTECODE=1

# Paths
GEN_SPEC_DIR="$PROJECT_ROOT/target/specs"
SPEC_IN="$PROJECT_ROOT/openapi.json"
# The spec as the generator sees it after its spec rules (webhook routing, string
# ids, union titles, ...; see scripts/generator-config.json). Written by the
# generator (preprocessedSpecFile) for debugging; no script reads it any more.
SPEC_OUT="$GEN_SPEC_DIR/l2_rust_compatible.json"
OUT_DIR="$PROJECT_ROOT"

# Ensure target directories exist
mkdir -p "$GEN_SPEC_DIR"
mkdir -p "$PROJECT_ROOT/target/reports"

echo "🚀 OpenAI Client Base Generation Pipeline"
echo "========================================="

# Check required tools before touching the checked-in specification.
if ! command -v uv &> /dev/null; then
    echo "❌ uv is not installed or not in PATH"
    echo "Please install uv from https://docs.astral.sh/uv/getting-started/installation/"
    exit 1
fi

# Step 1: Fetch and validate the current upstream specification.
# SKIP_SPEC_FETCH=1 regenerates from the checked-in openapi.json instead, so a
# generator change can be compared without upstream spec changes mixed in.
if [ "${SKIP_SPEC_FETCH:-0}" = "1" ]; then
    echo "📥 SKIP_SPEC_FETCH=1 → using the checked-in openapi.json"
else
    echo "📥 Fetching latest OpenAPI spec..."
    bash "$SCRIPT_DIR/fetch_spec.sh"
fi

# Step 2: Validate the spec. The spec decisions (webhook module, string ids,
# deepObject query parameters) are options of the generator, in
# scripts/generator-config.json.
echo ""
echo "🔧 Validating spec..."

# Check if Docker is available for generator
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed or not in PATH"
    echo "Please install Docker from https://www.docker.com/get-started"
    exit 1
fi

# Missing upstream schemas must be fixed upstream, never fabricated here.
uv run python "$SCRIPT_DIR/validate_spec.py" "$SPEC_IN"

# Step 3: Generate Rust client
echo ""
echo "🦀 Generating Rust client..."

# Clean generated directories but preserve src folder structure
find "$OUT_DIR/src/apis" "$OUT_DIR/src/models" -name "*.rs" -delete
rm -rf "$OUT_DIR/docs" 2>/dev/null || true

# Run the generator. Hand-maintained files (Cargo.toml, README.md, src/lib.rs,
# ...) are listed in .openapi-generator-ignore and left alone.
docker run --rm \
    -v "$PROJECT_ROOT:/local" \
    -u "$(id -u):$(id -g)" \
    -e JAVA_OPTS="-Dlog.level=${OPENAPI_GENERATOR_LOG_LEVEL}" \
    "$GENERATOR_IMAGE" generate \
    -i "/local/openapi.json" \
    -g "$GENERATOR_NAME" \
    -o "/local" \
    -c "/local/scripts/generator-config.json" \
    --skip-validate-spec

# Step 4: Self-test the remaining generation scripts. The generator renders
# unions and decides Default and Display itself, and writes only the imports a
# file uses. Nothing rewrites the generated code afterwards any more. PyYAML is
# only needed by the test that reads the GitHub workflow.
echo ""
echo "🧪 Testing generation scripts..."
uv run --with pyyaml python -m unittest discover -s "$SCRIPT_DIR/tests"

# Fail closed if the generated APIs do not cover every raw upstream path
# operation and webhook. This audit uses the fetched source before patches.
uv run python "$SCRIPT_DIR/check_openapi_coverage.py" \
    "$SPEC_IN" "$PROJECT_ROOT/src/apis" \
    --webhook-module open_ai_client_base_webhooks_api \
    --report "$PROJECT_ROOT/target/reports/openapi-coverage.json"

# Step 5: Format the generated code
echo ""
echo "🎨 Formatting generated code..."
cargo fmt --all

# Step 6: Build the crate
echo ""
echo "🔨 Building crate..."
if [ "${FULL_CARGO_CLEAN:-0}" = "1" ]; then
    echo "  FULL_CARGO_CLEAN=1 → performing full cargo clean"
    cargo clean
else
    # Default: light clean to refresh build-script outputs only
    echo "  Performing light clean: removing target/*/build to refresh build scripts"
    rm -rf "$PROJECT_ROOT/target"/*/build 2>/dev/null || true
fi
cargo build

echo ""
echo "✅ Generation complete!"
echo "   Generated code in: $OUT_DIR"
echo "   Preprocessed spec at: $SPEC_OUT"
