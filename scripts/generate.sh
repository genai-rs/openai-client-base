#!/usr/bin/env bash
set -euo pipefail

# Config
OPENAPI_GENERATOR_VERSION="${OPENAPI_GENERATOR_VERSION:-7.15.0}"
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

# Backup existing manifest so we can preserve curated dependency ranges
if [ -f "$OUT_DIR/Cargo.toml" ]; then
    cp "$OUT_DIR/Cargo.toml" "$OUT_DIR/Cargo.toml.original"
fi

# Clean previous generated code (preserve lib.rs if it has custom code)
if [ -f "$OUT_DIR/lib.rs" ]; then
    cp "$OUT_DIR/lib.rs" "$OUT_DIR/lib.rs.backup"
fi

# Clean generated directories but preserve src folder structure
find "$OUT_DIR" -name "*.rs" -not -name "lib.rs" -delete 2>/dev/null || true
rm -rf "$OUT_DIR/docs" 2>/dev/null || true

# Run OpenAPI Generator via Docker
GENERATOR_CMD="docker run --rm -v $PROJECT_ROOT:/local -u $(id -u):$(id -g) -e JAVA_OPTS=-Dlog.level=${OPENAPI_GENERATOR_LOG_LEVEL} openapitools/openapi-generator-cli:v${OPENAPI_GENERATOR_VERSION}"

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

# Step 6c: Fix Box<Value> constructor mismatches caused by generic fallbacks
echo ""
echo "🔧 Fixing Box<Value> constructor mismatches..."
uv run python scripts/fix_box_value_constructor_mismatches.py

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

# Step 8: Fix nullable fields and constructor signatures
echo ""
echo "🔧 Fixing nullable fields and constructors..."
if [ -f "$SCRIPT_DIR/fix_boxed_nullable_fields.py" ]; then
    uv run python scripts/fix_boxed_nullable_fields.py "$PROJECT_ROOT"
fi
if [ -f "$SCRIPT_DIR/fix_constructor_signatures.py" ]; then
    uv run --with pyyaml python scripts/fix_constructor_signatures.py "$PROJECT_ROOT"
fi
if [ -f "$SCRIPT_DIR/fix_option_box_constructor_mismatches.py" ]; then
    uv run python scripts/fix_option_box_constructor_mismatches.py "$PROJECT_ROOT"
fi

# Step 9: Fix Default trait issues
echo ""
echo "🔧 Fixing Default trait issues..."
if [ -f "$SCRIPT_DIR/fix_default_issues.py" ]; then
    uv run python scripts/fix_default_issues.py "$PROJECT_ROOT"
fi

# Re-run general code fixes to ensure Display/Default adjustments are present
uv run python scripts/fix_generated_code.py

# Step 10: Fix clippy warnings in generated code
echo ""
echo "🔧 Fixing clippy warnings..."
uv run python scripts/fix_clippy_warnings.py

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
echo "   Patched spec at: $SPEC_OUT"
echo ""
echo "Note: The following post-generation fixes were applied:"
echo "  - Fixed recursive types and compilation issues"
echo "  - Fixed empty enums and acronym casing (MCP -> Mcp, etc.)"
echo "  - Fixed untagged anyOf unions (content types, tool choice, etc.)"
echo "  - Added bon::Builder derives to all structs"
echo "  - Fixed clippy warnings with appropriate allow attributes"
echo "  - Applied cargo fmt for consistent formatting"
