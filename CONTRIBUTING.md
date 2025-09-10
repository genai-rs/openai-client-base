# Contributing to openai-client-base

Thank you for your interest in contributing! However, please note:

## ⚠️ Important: This is Generated Code

This crate is **automatically generated** from the OpenAI OpenAPI specification. Direct modifications to the generated code will be overwritten on the next generation cycle.

## How to Contribute

### For Code Issues

If you find issues with the generated code:

1. **DO NOT** modify the generated Rust code directly
2. Instead, contribute to the generation pipeline:
   - Fix issues in `scripts/fix_model_fields.py` (Layer 1 patches)
   - Fix issues in `scripts/patch_spec_rust_compat.py` (Layer 2 patches)
   - Fix issues in `scripts/patch_generated.rs.sh` (post-generation patches)

### For Documentation

Documentation improvements are welcome for:
- `README.md`
- `docs/GENERATION_PIPELINE.md`
- Comments in the patch scripts

### Submitting Changes

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-fix`)
3. Make your changes to the **generation pipeline**, not the generated code
4. Test by running `./scripts/generate.sh`
5. Commit your changes (`git commit -m 'fix: improve X in generation pipeline'`)
6. Push to your fork (`git push origin feature/amazing-fix`)
7. Open a Pull Request

### Testing

Before submitting:
```bash
# Generate the client
./scripts/generate.sh

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy -- -D warnings
```

Thank you for understanding the special nature of this generated crate!