#!/usr/bin/env bash
set -euo pipefail

# Config
OPENAPI_GENERATOR_VERSION="${OPENAPI_GENERATOR_VERSION:-7.15.0}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

SPEC_IN=${1:-stainless.yaml}
SPEC_OUT=${2:-target/specs/openapi.patched.yaml}
OUT_DIR=${3:-$PROJECT_ROOT}
PKG_NAME=${PKG_NAME:-openai_patched}

echo "[gen] Using OpenAPI Generator $OPENAPI_GENERATOR_VERSION with Docker"

# Check if Docker is available
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed or not in PATH"
    echo "Please install Docker from https://www.docker.com/get-started"
    exit 1
fi

# Use uv for Python dependencies if available, otherwise fallback to venv
echo "[gen] Processing spec through patching layers..."

# Intermediate files
SPEC_MODEL_FIXED="target/specs/openapi.model_fixed.yaml"

# Ensure target directories exist
mkdir -p "$PROJECT_ROOT/target/specs"

if command -v uv &> /dev/null; then
    echo "[gen] Using uv for Python dependencies"
    cd "$PROJECT_ROOT"
    
    # Layer 1: Fix model field types (handle allOf inheritance)
    echo "[gen] Layer 1: Fixing model field types..."
    uv run --with pyyaml python scripts/fix_model_fields.py "$SPEC_IN" "$SPEC_MODEL_FIXED"
    
    # Layer 2: Apply Rust compatibility patches
    echo "[gen] Layer 2: Applying Rust compatibility patches..."
    uv run --with pyyaml python scripts/patch_spec_rust_compat.py "$SPEC_MODEL_FIXED" "$SPEC_OUT"
else
    echo "[gen] Using Python venv for dependencies"
    VENV_DIR="$PROJECT_ROOT/.venv"
    if [ ! -d "$VENV_DIR" ]; then
        python3 -m venv "$VENV_DIR"
    fi
    source "$VENV_DIR/bin/activate"
    pip install pyyaml >/dev/null 2>&1
    
    # Layer 1: Fix model field types (handle allOf inheritance)
    echo "[gen] Layer 1: Fixing model field types..."
    python scripts/fix_model_fields.py "$SPEC_IN" "$SPEC_MODEL_FIXED"
    
    # Layer 2: Apply Rust compatibility patches
    echo "[gen] Layer 2: Applying Rust compatibility patches..."
    python scripts/patch_spec_rust_compat.py "$SPEC_MODEL_FIXED" "$SPEC_OUT"
    
    deactivate
fi

echo "[gen] Generating reqwest client into $OUT_DIR"

# Backup custom files if generating into subdirectory
if [ "$OUT_DIR" != "$PROJECT_ROOT" ]; then
    if [ -d "$OUT_DIR" ]; then
        echo "[gen] Backing up custom files..."
        if [ -d "$OUT_DIR/examples" ]; then
            cp -r "$OUT_DIR/examples" "$OUT_DIR/examples.backup" 2>/dev/null || true
        fi
    fi
    # Remove old generated code
    rm -rf "$OUT_DIR"
else
    # When generating into project root, only clean generated files
    echo "[gen] Cleaning generated files in project root..."
    rm -rf "$OUT_DIR/src/models" 2>/dev/null || true
    rm -rf "$OUT_DIR/src/apis" 2>/dev/null || true
    rm -rf "$OUT_DIR/docs" 2>/dev/null || true
    rm -f "$OUT_DIR/.openapi-generator-ignore" 2>/dev/null || true
    rm -rf "$OUT_DIR/.openapi-generator" 2>/dev/null || true
fi

# Run Docker generator with proper user permissions
GENERATOR_CMD="docker run --rm -v $PROJECT_ROOT:/local -u $(id -u):$(id -g) openapitools/openapi-generator-cli:v${OPENAPI_GENERATOR_VERSION}"

$GENERATOR_CMD generate \
    -i "/local/$SPEC_OUT" \
    -g rust \
    -o "/local/$OUT_DIR" \
    --skip-validate-spec \
    --additional-properties=packageName=$PKG_NAME \
    --additional-properties=packageVersion=0.1.0 \
    --additional-properties=library=reqwest \
    --additional-properties=supportAsync=true \
    --additional-properties=supportMiddleware=true \
    --additional-properties=preferUnsignedInt=false \
    --additional-properties=useSingleRequestParameter=false

echo "[gen] Patching generated sources (minor fixes)"
bash "$SCRIPT_DIR/patch_generated.rs.sh" "$OUT_DIR"

# Restore custom examples if they existed
if [ -d "$OUT_DIR/examples.backup" ]; then
    echo "[gen] Restoring custom examples..."
    mv "$OUT_DIR/examples.backup" "$OUT_DIR/examples"
fi

echo "[gen] Building crate"
(cd "$OUT_DIR" && cargo build)

# Copy notes + patch report
cp -f docs/GEN-NOTES.md "$OUT_DIR"/GEN-NOTES.md 2>/dev/null || true
cp -f target/specs/patch_report.md "$OUT_DIR"/PATCH-REPORT.md 2>/dev/null || true

echo "[gen] Done. Crate at $OUT_DIR"