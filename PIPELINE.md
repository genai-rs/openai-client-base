# OpenAI Client Generation Pipeline

This document describes the automated generation pipeline for the OpenAI Rust client.

## Overview

The pipeline fetches the latest OpenAPI specification from Stainless and generates a Rust client with all necessary fixes applied automatically.

## Pipeline Steps

### 1. Fetch Specification (`fetch_spec.sh`)
- Downloads the latest OpenAPI spec from Stainless API
- Saves as `stainless.yaml`

### 2. Apply Spec Patches (Layer 1 & 2)
- **Layer 1: Model Field Fixes** (`fix_model_fields.py`)
  - Fixes allOf inheritance issues
  - Flattens model field definitions
  
- **Layer 2: Rust Compatibility** (`patch_spec_rust_compat.py`)
  - Simplifies union types
  - Fixes multipart form handling
  - Renames fields that conflict with Rust keywords

### 3. Generate Rust Code
- Uses OpenAPI Generator via Docker
- Generates client with reqwest library

### 4. Apply Post-Generation Patches (`patch_generated.rs.sh`)
- Fixes module paths
- Adds bon builder support
- Fixes content types

### 5. Fix Compilation Issues (`fix_generated_code.py`)
- Fixes invalid enum variant names (e.g., Gpt4.1 → Gpt4_1)
- Adds Display implementations for multipart types
- Removes Default from empty enums
- Handles recursive type issues

### 6. Fix Empty Enums (`fix_empty_enums.py`)
- Populates empty enums with proper variants from spec

### 7. Fix Untagged Unions (`fix_untagged_unions.py`)
- Automatically detects untagged unions from spec
- Adds #[serde(untagged)] attribute
- Creates proper enum variants

### 8. Fix Nullable Fields (`fix_boxed_nullable_fields.py`, `fix_constructor_signatures.py`, `fix_option_box_constructor_mismatches.py`)
- Converts Box<T> to Option<Box<T>> for nullable fields
- Updates constructor signatures to match
- Normalizes constructors for Option<Box<T>> fields even when spec lacks `nullable`

### 9. Fix Default Trait Issues (`fix_default_issues.py`)
- Removes Default derive from structs with non-Default fields

### 10. Fix Clippy Warnings (`fix_clippy_warnings.py`)
- Addresses common clippy lints

### 11. Format Code
- Runs cargo fmt for consistent formatting

### 12. Build
- Performs final compilation check

## Key Fixes Applied

### Automatic Detection
- Untagged unions are detected from anyOf/oneOf without discriminators
- Nullable fields are identified and properly wrapped in Option
- Empty enums are populated from spec

### Rust-Specific Issues
- Enum variant names sanitized (no dots allowed)
- Display trait added where needed for multipart forms
- Default trait managed based on field types
- Module paths corrected

## Running the Pipeline

```bash
# Full generation from scratch
./scripts/generate.sh

# Use cached spec (skip download)
USE_CACHED_SPEC=1 ./scripts/generate.sh

# Control OpenAPI Generator verbosity (error|warn|info|debug|trace)
OPENAPI_GENERATOR_LOG_LEVEL=error ./scripts/generate.sh
```

## Adding New Fixes

1. For spec-level fixes: Add to `patch_spec_rust_compat.py`
2. For post-generation fixes: Add to `fix_generated_code.py`
3. For new fix scripts: Add to `generate.sh` pipeline

## Dependencies

- Docker (for OpenAPI Generator)
- uv (for Python dependencies)
- Rust toolchain
- ripgrep (rg)
