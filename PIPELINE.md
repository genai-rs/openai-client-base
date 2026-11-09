# OpenAI Client Generation Pipeline

`scripts/generate.sh` regenerates the crate from OpenAI's published OpenAPI
specification. The Rust code comes from the
[`timvw-rust`](https://github.com/timvw/openapi-generator-timvw-rust) generator;
no script edits the generated code afterwards.

## Steps

1. **Fetch** (`fetch_spec.sh`): downloads the current spec from
   [OpenAI's published repository](https://github.com/openai/openai-openapi/blob/main/openapi.yaml)
   into `stainless.yaml`. HTTP errors, malformed documents and unresolved local
   references are rejected before the checked-in copy is replaced. Set
   `SKIP_SPEC_FETCH=1` to regenerate from the checked-in copy.
2. **Validate** (`validate_spec.py`) and convert the spec to JSON. Missing
   upstream schemas stop the run; they are fixed upstream, never added here.
3. **Generate** with the `timvw-rust` image
   (`ghcr.io/timvw/openapi-generator-timvw-rust:<TIMVW_RUST_VERSION>-og<OPENAPI_GENERATOR_VERSION>`).
   Every decision the crate takes about the spec and its rendering is a
   generator option in `scripts/generator-config.json`: webhook module, string
   ids, deep-object query parameters, multipart handling,
   lint allows, typed event streams (`typedEventStreams`) and one method per
   media type (`contentTypeMethods`). Error boxing, boxing only on cycles,
   unions and the other baseline renderings are always on in timvw-rust 0.21.0. The generator writes the spec after its
   spec rules to `target/specs/l2_rust_compatible.json`. Hand-maintained files
   (`Cargo.toml`, `README.md`, `src/lib.rs`, ...) are listed in
   `.openapi-generator-ignore` and left alone.
4. **Check**: the remaining script tests (`scripts/tests`), and
   `check_openapi_coverage.py`, which fails when the generated APIs miss an
   upstream operation or webhook.
5. **Format** with `cargo fmt` and **build** with `cargo build`.

After generation, run `cargo clippy --all-targets --all-features -- -D warnings`
and `cargo test --all-features`.

## Public API changes

Pull requests that touch the pipeline compare the crate's public API with their
base commit (`check_public_api.sh`). A change fails the check unless the pull
request carries the `api-change` label; breaking changes get a note under
`migration/`.

## Running the pipeline

```bash
# Full generation, fetching the current spec
./scripts/generate.sh

# Regenerate from the checked-in stainless.yaml
SKIP_SPEC_FETCH=1 ./scripts/generate.sh

# Use a locally built generator image
GENERATOR_IMAGE=timvw-rust:dev ./scripts/generate.sh

# openapi-generator verbosity (error|warn|info|debug|trace)
OPENAPI_GENERATOR_LOG_LEVEL=error ./scripts/generate.sh

# Full cargo clean instead of the default light clean of build-script outputs
FULL_CARGO_CLEAN=1 ./scripts/generate.sh
```

## Changing the generated code

Change the generator, not the output: add or adjust a rule or option in
`timvw-rust`, release it, and bump `TIMVW_RUST_VERSION` in `generate.sh`.

## Dependencies

- Docker (runs the generator image)
- uv (Python for the spec checks)
- Rust toolchain
