#!/usr/bin/env bash
set -euo pipefail
ROOT=${1:-crates/openai-patched}

# 1) Fix double module path `models::models::` → `models::`
rg -l "models::models::" "$ROOT/src" 2>/dev/null | while read -r f; do
  sed -i.bak 's/models::models::/models::/g' "$f" && rm -f "$f.bak"
done

# 2) Loosen content types only for select non-chat models; keep chat message content enums intact
for f in \
  "$ROOT/src/models/create_message_request.rs" \
  "$ROOT/src/models/prediction_content.rs"; do
  if [ -f "$f" ]; then
    sed -i.bak 's/pub content: String,/pub content: serde_json::Value,/' "$f"
    sed -i.bak 's/new(content: String,/new(content: serde_json::Value,/' "$f" || true
    # If new() still builds from String, wrap into Value::String
    sed -i.bak 's/\(pub fn new(.*content: \)serde_json::Value,\(.*\)\n[[:space:]]*{\n[[:space:]]*\(.*\)content,/\1serde_json::Value,\2\n        {\n            \3content: content,/' "$f" || true
    sed -i.bak 's/\(pub fn new(.*content: \)String,\(.*\)\n[[:space:]]*{\n[[:space:]]*\(.*\)content,/\1String,\2\n        {\n            \3content: serde_json::Value::String(content),/' "$f" || true
    rm -f "$f.bak"
  fi
done

# Explicit constructor fixes
cf="$ROOT/src/models/create_message_request.rs"
if [ -f "$cf" ]; then
  sed -i.bak 's/pub fn new(role: Role, content: String)/pub fn new(role: Role, content: serde_json::Value)/' "$cf"
  rm -f "$cf.bak"
fi
cf="$ROOT/src/models/prediction_content.rs"
if [ -f "$cf" ]; then
  sed -i.bak 's/pub fn new(r#type: Type, content: String)/pub fn new(r#type: Type, content: serde_json::Value)/' "$cf"
  rm -f "$cf.bak"
fi

# Certificates API left untouched; fixed by spec param renaming

# 3) Fix any lingering `to_string()` on non-Display query types by casting to JSON string
# Target common patterns: `&local_var_str.to_string()` → `&local_var_str` when already a &str
# and object maps, but we’ll keep it minimal here.

echo "Patched generated Rust sources under $ROOT"

# Apply multipart file upload fixes
bash scripts/patch_multipart.sh "$ROOT"

# 3) Add bon dependency to generated crate for builder macros
toml="$ROOT/Cargo.toml"
if [ -f "$toml" ] && ! grep -q '^bon[[:space:]]*=' "$toml"; then
  # insert under [dependencies]
  awk 'BEGIN{added=0} /\[dependencies\]/{print; print "bon = \"3\""; added=1; next} {print} END{if(added==0) print "[dependencies]\nbon = \"3\""}' "$toml" > "$toml.tmp" && mv "$toml.tmp" "$toml"
fi
# add dev-dependencies for examples
if [ -f "$toml" ] && ! grep -q '^\[dev-dependencies\]' "$toml"; then
  printf "\n[dev-dependencies]\nanyhow = \"1\"\ntokio = { version = \"1\", features = [\"rt-multi-thread\", \"macros\"] }\n" >> "$toml"
fi

# 4) Add #[bon::builder] to all free API functions for compile-time checked builders
for f in "$ROOT"/src/apis/*.rs; do
  [ -f "$f" ] || continue
  awk 'NR==1{prev=""} { 
    if ($0 ~ /^pub async fn /) {
      if (prev !~ /bon::builder/) { print "#[bon::builder]" }
      print $0
    } else { print $0 }
    prev=$0
  }' "$f" > "$f.tmp" && mv "$f.tmp" "$f"
done

# 5) Add bon::Builder derive to ALL structs using Python script
# Check if uv is available
if ! command -v uv &> /dev/null; then
    echo "❌ uv is not installed or not in PATH"
    echo "Please install uv from https://docs.astral.sh/uv/getting-started/installation/"
    echo "Quick install: curl -LsSf https://astral.sh/uv/install.sh | sh"
    exit 1
fi
uv run python scripts/add_bon_builders.py "$ROOT"
