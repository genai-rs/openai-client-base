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
- `PIPELINE.md`
- Comments in the patch scripts

### Submitting Changes

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-fix`)
3. Make your changes to the **generation pipeline**, not the generated code
4. Test by running `./scripts/generate.sh`
5. Commit your changes (`git commit -m 'fix: improve X in generation pipeline'`)
6. Push to your fork (`git push origin feature/amazing-fix`)
7. Open a Pull Request

### Public API check

`scripts/check_public_api.sh [git-ref]` compares the public Rust declarations in `src/` (model exports, struct fields, enum variants, function signatures, derives, serde attributes) against a reference git ref, by default the latest tag. It exits 1 on any difference unless `ALLOW_API_CHANGES=1` is set, and writes the report to `target/reports/public-api-diff.json`.

CI runs it on every PR, but it only *fails* a PR that changes the generation pipeline (`scripts/`, `migration/`, the generate workflow): such a change must reproduce the released API, or ship the diff intentionally: label the PR `api-change`, add the report as `migration/api-diff-<from>-to-<to>.json` and a migration note next to it (see `migration/api-diff-0.16.1-to-0.17.0.md`). Spec-driven regeneration PRs are expected to move the API; for them the job is report-only and the review happens through the api-change report of `generate-client.yml`.

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