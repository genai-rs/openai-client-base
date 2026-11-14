# Contributing to openai-client-base

Thank you for your interest in contributing! However, please note:

## ⚠️ Important: This is Generated Code

This crate is **automatically generated** from the OpenAI OpenAPI specification. Direct modifications to the generated code will be overwritten on the next generation cycle.

## How to Contribute

### For Code Issues

If you find issues with the generated code:

1. **DO NOT** modify the generated Rust code directly
2. Instead, contribute to the generation pipeline:
   - Spec decisions (webhook module, schemas and properties rendered as `String`, deepObject query parameters) are options of the generator in `scripts/generator-config.json`
   - Rendering rules belong in the generator itself, [timvw-rust](https://github.com/timvw/openapi-generator-timvw-rust)

### For Documentation

Documentation improvements are welcome for:
- `README.md`
- the guides in `guides/`
- Comments in the generation scripts

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

CI runs it on every PR, but it only *fails* a PR that changes the generation pipeline (`scripts/`, `migration/`, the generate workflow): such a change is compared against the PR's base commit and must reproduce that API, or ship the diff intentionally: label the PR `api-change`, add the report as `migration/api-diff-<from>-to-<to>.json` and a migration note next to it (see `migration/api-diff-0.16.1-to-0.17.0.md`). Spec-driven regeneration PRs are expected to move the API; for them the job is report-only and the review happens through the api-change report of `generate-client.yml`.

### Testing

Before submitting:
```bash
# Generate the client
./scripts/generate.sh

# Run the generation script tests
uv run --with pyyaml python -m unittest discover -s scripts/tests

# Run tests
cargo test --all-features

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings
```

`generate.sh` needs Docker and [uv](https://docs.astral.sh/uv/); see [Generation](guides/generation.md).

Thank you for understanding the special nature of this generated crate!