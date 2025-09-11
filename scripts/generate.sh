#!/usr/bin/env bash
set -euo pipefail

# Config
OPENAPI_GENERATOR_VERSION="${OPENAPI_GENERATOR_VERSION:-7.15.0}"
PKG_NAME="openai_client_base"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Paths
GEN_SPEC_DIR="$PROJECT_ROOT/target/specs"
SPEC_IN="$PROJECT_ROOT/stainless.yaml"
SPEC_MODEL_FIXED="$GEN_SPEC_DIR/l1_model_fixed.yaml"
SPEC_OUT="$GEN_SPEC_DIR/l2_rust_compatible.yaml"
OUT_DIR="$PROJECT_ROOT"

# Ensure target directories exist
mkdir -p "$GEN_SPEC_DIR"
mkdir -p "$PROJECT_ROOT/target/reports"

echo "🚀 OpenAI Client Base Generation Pipeline"
echo "========================================="

# Step 1: Fetch latest spec unless using cached version
if [ "${USE_CACHED_SPEC:-0}" = "1" ] && [ -f "$SPEC_IN" ]; then
    echo "✓ Using cached spec at $SPEC_IN"
else
    echo "📥 Fetching latest OpenAPI spec..."
    bash "$SCRIPT_DIR/fetch_spec.sh"
fi

# Step 2: Apply patching pipeline
echo ""
echo "🔧 Applying spec patches..."

# Check if Docker is available for generator
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed or not in PATH"
    echo "Please install Docker from https://www.docker.com/get-started"
    exit 1
fi

# Check if uv is available
if ! command -v uv &> /dev/null; then
    echo "❌ uv is not installed or not in PATH"
    echo "Please install uv from https://docs.astral.sh/uv/getting-started/installation/"
    echo "Quick install: curl -LsSf https://astral.sh/uv/install.sh | sh"
    exit 1
fi

echo "Using uv for Python dependencies"

# Layer 1: Fix model field types (handle allOf inheritance)
echo "  Layer 1: Fixing model field types..."
uv run --with pyyaml python "$SCRIPT_DIR/fix_model_fields.py" "$SPEC_IN" "$SPEC_MODEL_FIXED"

# Layer 2: Apply Rust compatibility patches
echo "  Layer 2: Applying Rust compatibility patches..."
uv run --with pyyaml python "$SCRIPT_DIR/patch_spec_rust_compat.py" "$SPEC_MODEL_FIXED" "$SPEC_OUT"

# Step 3: Generate Rust client
echo ""
echo "🦀 Generating Rust client..."

# Clean previous generated code (preserve lib.rs if it has custom code)
if [ -f "$OUT_DIR/lib.rs" ]; then
    cp "$OUT_DIR/lib.rs" "$OUT_DIR/lib.rs.backup"
fi

# Clean generated directories but preserve src folder structure
find "$OUT_DIR" -name "*.rs" -not -name "lib.rs" -delete 2>/dev/null || true
rm -rf "$OUT_DIR/docs" 2>/dev/null || true

# Run OpenAPI Generator via Docker
GENERATOR_CMD="docker run --rm -v $PROJECT_ROOT:/local -u $(id -u):$(id -g) openapitools/openapi-generator-cli:v${OPENAPI_GENERATOR_VERSION}"

$GENERATOR_CMD generate \
    -i "/local/target/specs/l2_rust_compatible.yaml" \
    -g rust \
    -o "/local" \
    --skip-validate-spec \
    --additional-properties=packageName=$PKG_NAME \
    --additional-properties=packageVersion=0.1.0 \
    --additional-properties=library=reqwest \
    --additional-properties=supportAsync=true \
    --additional-properties=supportMiddleware=true \
    --additional-properties=preferUnsignedInt=false \
    --additional-properties=useSingleRequestParameter=false

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

# Step 5: Fix compilation issues in generated code
echo ""
echo "🔧 Fixing compilation issues..."
uv run python scripts/fix_generated_code.py

# Step 6: Fix empty enums that should have variants
echo ""
echo "🔧 Fixing empty enums..."
uv run --with pyyaml python scripts/fix_empty_enums.py

# Step 7: Fix untagged anyOf unions that generate as empty structs
echo ""
echo "🔧 Fixing untagged unions..."
uv run --with pyyaml python scripts/fix_untagged_unions.py "$PROJECT_ROOT" "$SPEC_OUT"

# Step 8: Fix clippy warnings in generated code
echo ""
echo "🔧 Fixing clippy warnings..."
uv run python scripts/fix_clippy_warnings.py

# Step 9: Format the generated code
echo ""
echo "🎨 Formatting generated code..."
cargo fmt --all

# Step 10: Build the crate
echo ""
echo "🔨 Building crate..."
echo "  Cleaning build cache to avoid corruption issues..."
cargo clean
cargo build

echo ""
echo "✅ Generation complete!"
echo "   Generated code in: $OUT_DIR"
echo "   Patched spec at: $SPEC_OUT"
echo ""
echo "Note: The following post-generation fixes were applied:"
echo "  - Fixed recursive types and compilation issues"
echo "  - Fixed empty enums and acronym casing (MCP -> Mcp, etc.)"
echo "  - Fixed untagged anyOf unions (content types, tool choice, etc.)"
echo "  - Added bon::Builder derives to all structs"
echo "  - Fixed clippy warnings with appropriate allow attributes"
echo "  - Applied cargo fmt for consistent formatting"