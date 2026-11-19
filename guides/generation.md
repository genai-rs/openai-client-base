# Generation

`scripts/generate.sh` regenerates the crate from OpenAI's published OpenAPI
specification. The Rust code comes from the
[`timvw-rust`](https://github.com/timvw/openapi-generator-timvw-rust) generator;
no script edits the generated code afterwards, and the spec is passed to the
generator as OpenAI publishes it.

The pipeline in short: fetch the spec → validate it → generate with timvw-rust
→ `cargo fmt` → check coverage (every operation and webhook has a function) and,
in CI, the public API against the last release.

## What is generated and what is not

Generated, and replaced on every run: `crates/`, `src/apis/mod.rs`,
`src/models/mod.rs` and `docs/` (the generated reference pages for every API
and model). Hand-maintained, and
listed in `.openapi-generator-ignore` or outside the generator's output:
`Cargo.toml`, `src/lib.rs`, `README.md`, these guides (`guides/`), `tests/`,
`scripts/` and `migration/`. The guides live in `guides/` rather than under
`docs/` because the pipeline deletes `docs/` before generating.

## The crates of the workspace

The crate is generated as a Cargo workspace (timvw-rust `splitCrates`, see its
[guide](https://github.com/timvw/openapi-generator-timvw-rust/blob/main/docs/guides/split-crates.md)):
as one crate, rustc and rustdoc needed 11–12 GB, more than docs.rs and many
machines have. `openai-client-base`, at the root, is a facade with no code of
its own: `src/models/mod.rs` and `src/apis/mod.rs` re-export the crates under
`crates/`, under the paths and Cargo features the single crate had.

| Crate | Holds |
| --- | --- |
| `openai-client-base-common` | The models several crates use, `configuration`, `Error`, `ResponseContent`, `ApiResponse`, `sse`, `validation`, the helpers |
| `openai-client-base-<crate>` | The models and API modules of the groups of `scripts/api-features.yaml` with that `crate:` |
| `openai-client-base-untagged` | The models no operation uses (feature `types`) |

The `crate:` of each group in `scripts/api-features.yaml` decides the crates:
groups that share models share a crate, and each crate stays well under ~100k
generated lines (`responses`, the largest group, is a crate of its own). A
model a second crate starts to use moves to `common`; that changes nothing for
users of the facade. The other crates are internal: users depend on
`openai-client-base` only.

The root `Cargo.toml` is hand-maintained (`.openapi-generator-ignore`) and holds
the workspace: `[workspace.package]` has the version every crate inherits,
`[workspace.dependencies]` pins each crate with `=` to that version, and
`[workspace.lints]` holds the lint allows of every crate. When the crates
change (a new `crate:`, a group added), copy the generator's `[features]`,
`[dependencies]`, `[workspace] members` and `[workspace.dependencies]` (generate
once without `.openapi-generator-ignore` into another directory), and list the
new crate in `release-plz.toml`.

Releases publish every crate with the same version (one release-plz
`version_group`), dependencies first; only the facade has a changelog, tags and
GitHub releases. A new crate needs a first publish by hand (with a token) and a
Trusted Publisher on crates.io before release-plz can publish it.

## Steps

1. **Fetch** (`fetch_spec.sh`): downloads the current spec from
   [OpenAI's published repository](https://github.com/openai/openai-openapi/blob/main/openapi.json)
   into `openapi.json`. HTTP errors, malformed documents and unresolved local
   references are rejected before the checked-in copy is replaced. Set
   `SKIP_SPEC_FETCH=1` to regenerate from the checked-in copy.
2. **Validate** (`validate_spec.py`). Missing
   upstream schemas stop the run; they are fixed upstream, never added here.
3. **Generate** with the `timvw-rust` image
   (`ghcr.io/timvw/openapi-generator-timvw-rust:<TIMVW_RUST_VERSION>-og<OPENAPI_GENERATOR_VERSION>`).
   Every decision the crate takes about the spec and its rendering is a
   generator option in `scripts/generator-config.json`: webhook module, string
   ids, deep-object query parameters, multipart handling,
   lint allows, typed event streams (`typedEventStreams`) and one method per
   media type (`contentTypeMethods`). Error boxing, boxing only on cycles,
   unions and the other baseline renderings are always on since timvw-rust 0.21.0. The generator writes the spec after its
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

# Regenerate from the checked-in openapi.json
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

## Requirements

- Docker (runs the generator image)
- [uv](https://docs.astral.sh/uv/getting-started/installation/) (Python for the spec checks and script tests)
- A Rust toolchain (stable, for formatting)
