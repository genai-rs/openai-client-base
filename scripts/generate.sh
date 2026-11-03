#!/usr/bin/env bash
set -euo pipefail

# Config
OPENAPI_GENERATOR_VERSION="${OPENAPI_GENERATOR_VERSION:-7.25.0}"
# Custom generator image (timvw-rust extends the stock rust generator; see
# https://github.com/timvw/openapi-generator-timvw-rust). Image tags are
# <generator version>-og<openapi-generator version>. Override for local builds,
# e.g. GENERATOR_IMAGE=timvw-rust:dev.
TIMVW_RUST_VERSION="${TIMVW_RUST_VERSION:-0.7.0}"
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
SPEC_IN="$PROJECT_ROOT/stainless.yaml"
# JSON form of the spec fed to the generator: the OpenAPI 3.1 $ref dereferencer
# parses YAML through snakeyaml, which aborts once the document exceeds 3145728
# code points ("The incoming YAML document exceeds the limit"). The Stainless
# spec has grown past that; JSON parsing has no such limit.
SPEC_IN_JSON="$GEN_SPEC_DIR/stainless.json"
# The spec as the generator sees it after its spec rules (webhook routing, string
# ids, union titles, ...; see scripts/generator-config.json). Written by the
# generator (preprocessedSpecFile) for the union scripts that still read it.
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
# SKIP_SPEC_FETCH=1 regenerates from the checked-in stainless.yaml instead, so a
# generator change can be compared without upstream spec changes mixed in.
if [ "${SKIP_SPEC_FETCH:-0}" = "1" ]; then
    echo "📥 SKIP_SPEC_FETCH=1 → using the checked-in stainless.yaml"
else
    echo "📥 Fetching latest OpenAPI spec..."
    bash "$SCRIPT_DIR/fetch_spec.sh"
fi

# Step 2: Validate the spec and hand it to the generator as JSON. The spec
# decisions (webhook module, string ids, free-form schemas, deepObject query
# parameters) are options of the generator, in scripts/generator-config.json.
echo ""
echo "🔧 Preparing spec..."

# Check if Docker is available for generator
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed or not in PATH"
    echo "Please install Docker from https://www.docker.com/get-started"
    exit 1
fi

echo "Using uv for Python dependencies"

# Missing upstream schemas must be fixed upstream, never fabricated here.
echo "  Validating OpenAPI spec..."
uv run --with pyyaml python "$SCRIPT_DIR/validate_spec.py" "$SPEC_IN"

echo "  Converting spec to JSON..."
uv run --with pyyaml python -c "import sys, json, yaml; json.dump(yaml.safe_load(open(sys.argv[1])), open(sys.argv[2], 'w'), default=str)" "$SPEC_IN" "$SPEC_IN_JSON"

# Step 3: Generate Rust client
echo ""
echo "🦀 Generating Rust client..."

# Backup existing manifest so we can preserve curated dependency ranges
if [ -f "$OUT_DIR/Cargo.toml" ]; then
    cp "$OUT_DIR/Cargo.toml" "$OUT_DIR/Cargo.toml.original"
fi

# Clean previous generated code (preserve lib.rs if it has custom code)
if [ -f "$OUT_DIR/lib.rs" ]; then
    cp "$OUT_DIR/lib.rs" "$OUT_DIR/lib.rs.backup"
fi

# Clean generated directories but preserve src folder structure
find "$OUT_DIR/src/apis" "$OUT_DIR/src/models" -name "*.rs" -delete
rm -rf "$OUT_DIR/docs" 2>/dev/null || true

# Run OpenAPI Generator via Docker
docker run --rm \
    -v "$PROJECT_ROOT:/local" \
    -u "$(id -u):$(id -g)" \
    -e JAVA_OPTS="-Dlog.level=${OPENAPI_GENERATOR_LOG_LEVEL}" \
    "$GENERATOR_IMAGE" generate \
    -i "/local/target/specs/stainless.json" \
    -g "$GENERATOR_NAME" \
    -o "/local" \
    -c "/local/scripts/generator-config.json" \
    --skip-validate-spec

echo ""
echo "📝 Merging Cargo.toml dependency updates..."
if [ -f "$OUT_DIR/Cargo.toml.original" ]; then
    mv "$OUT_DIR/Cargo.toml" "$OUT_DIR/Cargo.toml.generated"
    mv "$OUT_DIR/Cargo.toml.original" "$OUT_DIR/Cargo.toml"

    python3 "$SCRIPT_DIR/merge_cargo_dependencies.py" \
        "$OUT_DIR/Cargo.toml" \
        "$OUT_DIR/Cargo.toml.generated"

    rm -f "$OUT_DIR/Cargo.toml.generated"
fi

# Ensure no backups linger if the generator didn't touch the manifest
rm -f "$OUT_DIR/Cargo.toml.original"

# Step 4: Apply post-generation patches
echo ""
echo "🔨 Applying post-generation patches..."
bash "$SCRIPT_DIR/patch_generated.rs.sh" "$PROJECT_ROOT"

# Restore lib.rs if needed
if [ -f "$OUT_DIR/lib.rs.backup" ]; then
    # Check if lib.rs was regenerated
    if [ -f "$OUT_DIR/lib.rs" ]; then
        echo "Note: lib.rs was regenerated, backup available at lib.rs.backup"
    else
        mv "$OUT_DIR/lib.rs.backup" "$OUT_DIR/lib.rs"
    fi
fi

# Step 4b: Fix file upload TODOs (optional binary parameters)
echo ""
echo "🔧 Fixing file upload TODOs..."
uv run python scripts/fix_file_upload_todos.py

# Step 5: Fix compilation issues in generated code
echo ""
echo "🔧 Fixing compilation issues..."
uv run python scripts/fix_generated_code.py

# Step 6: Fix empty enums that should have variants
echo ""
echo "🔧 Fixing empty enums..."
uv run --with pyyaml python scripts/fix_empty_enums.py

# Step 6b: Normalize enum derives and misplaced attributes
echo ""
echo "🔧 Normalizing enum derives/attributes..."
uv run python scripts/fix_enum_attributes.py

# Step 6d: Fix tool-related enums with conflicting tagged serialization
# This must run BEFORE fix_untagged_unions.py to handle discriminated unions with field conflicts
echo ""
echo "🔧 Fixing tool-related enum tagging conflicts..."
uv run python scripts/fix_tool_enum_tagging.py "$PROJECT_ROOT"

# Step 7: Fix untagged anyOf unions that generate as empty structs
echo ""
echo "🔧 Fixing untagged unions..."
uv run --with pyyaml python scripts/fix_untagged_unions.py "$PROJECT_ROOT" "$SPEC_OUT"

# Remove helper impls that reference nonexistent variants after all enum fixes
echo ""
echo "🔧 Removing mismatched enum helper impls..."
uv run python scripts/fix_helper_impl_mismatches.py

# Fix enum variants that reference types simplified to strings (generic solution)
# This handles ModelIdsResponses, VoiceIdsShared, and all other simplified types
echo ""
echo "🔧 Fixing enum variants referencing simplified types..."
uv run python scripts/fix_simplified_type_references.py "$SCRIPT_DIR/generator-config.json"

# Step 9: Fix Default trait issues
echo ""
echo "🔧 Fixing Default trait issues..."
if [ -f "$SCRIPT_DIR/fix_default_issues.py" ]; then
    uv run python scripts/fix_default_issues.py "$PROJECT_ROOT"
fi

# Re-run general code fixes to ensure Display/Default adjustments are present
uv run python scripts/fix_generated_code.py

# Step 10: Self-test the remaining generation scripts
echo ""
echo "🧪 Testing generation scripts..."
uv run --with pyyaml python -m unittest discover -s "$SCRIPT_DIR/tests"

# Resolve generator-only Rust names and helpers after every transformation.
uv run python scripts/fix_generator_artifacts.py
uv run python scripts/fix_helper_impl_mismatches.py
uv run python scripts/fix_generated_code.py

# Apply multipart array, object Content-Type, and explicit encoding rules from the source spec.
uv run --with pyyaml python "$SCRIPT_DIR/fix_multipart_serialization.py" \
    "$SPEC_IN" "$PROJECT_ROOT/src/apis" \
    --report "$PROJECT_ROOT/target/reports/multipart-serialization.json"

# Re-apply the truncated-doc-link fix of patch_generated.rs.sh (#7): the union
# rewrites above regenerate model files from the spec after that patch ran, and
# rustdoc -D warnings rejects the unterminated "[text](https://..." it restores.
find "$PROJECT_ROOT/src" -name '*.rs' -print0 | xargs -0 perl -pi -e 'if (m{^\s*//[/!]}) { s{(\]\(https?://[^)\s]*)$}{$1)} }'

# Drop imports the generator emits but the file never uses (replaces the
# crate-wide allow(unused_imports)).
uv run python scripts/fix_unused_imports.py "$PROJECT_ROOT"

# Fail closed if the generated APIs do not cover every raw upstream path
# operation and webhook. This audit uses the fetched source before patches.
uv run --with pyyaml python "$SCRIPT_DIR/check_openapi_coverage.py" \
    "$SPEC_IN" "$PROJECT_ROOT/src/apis" \
    --webhook-module open_ai_client_base_webhooks_api \
    --report "$PROJECT_ROOT/target/reports/openapi-coverage.json"

# Step 11: Format the generated code
echo ""
echo "🎨 Formatting generated code..."
cargo fmt --all

# Step 12: Build the crate
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
echo ""
echo "Note: The following post-generation fixes were applied:"
echo "  - Fixed recursive types and compilation issues"
echo "  - Fixed empty enums and acronym casing (MCP -> Mcp, etc.)"
echo "  - Fixed untagged anyOf unions (content types, tool choice, etc.)"
echo "  - Added bon::Builder derives to all structs"
echo "  - Applied cargo fmt for consistent formatting"
