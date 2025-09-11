# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.3](https://github.com/genai-rs/openai-client-base/compare/v0.1.2...v0.1.3) - 2025-09-11

### Other

- post-step to sync GitHub release notes from CHANGELOG.md (ensures populated notes)

## [0.1.2](https://github.com/genai-rs/openai-client-base/compare/v0.1.1...v0.1.2) - 2025-09-11

### Other

- make sccache non-fatal and avoid GitHub artifact cache; enable only if server starts
- allow optional PAT via GH_RELEASE_TOKEN to create tags/releases (work around 403 on protected tags)
- add concurrency to generate-client and release-plz workflows (cancel older runs)
- add concurrency to generate-client and release-plz workflows (cancel older runs)

## [0.1.1](https://github.com/genai-rs/openai-client-base/compare/v0.1.0...v0.1.1) - 2025-09-11

### Other

- *(Cargo.toml)* add documentation link to docs.rs
- *(release-plz)* fix GitHub release body template to use per-release changelog; remove workflow changelog patch step
- release version 0.1.0

## [0.1.0](https://github.com/genai-rs/openai-client-base/releases/tag/v0.1.0) - 2025-09-11

### Added

- auto-detect untagged unions from OpenAPI spec
- add release automation and repository configuration
- add cargo fmt to generation pipeline
- upgrade to latest version of openai spec

### Fixed

- Consolidate post-generation fixes into automated pipeline
- handle nullable fields and Default trait implementation
- improve code generation pipeline and reduce compilation errors
- improve untagged unions script with proper casing and type resolution
- sanitize variant names in untagged unions to be valid Rust identifiers
- add proper Python environment handling for all fix scripts in CI
- convert untagged anyOf unions to proper Rust enums
- suppress clippy warnings in generated code
- update enum variant names to use proper Rust casing conventions
- make enum acronym handling generic and future-proof
- all empty enums are patched now
- empty enums
- empty enums
- add missed failes
- attempts to make ci work again
- typo
- also update newly expected helper functions
- ptimize patching

### Other

- *(README)* switch license badge to GitHub license badge to match langfuse style
- *(README)* fix license badge to static MIT/Apache-2.0 since crate not yet on crates.io
- enable release-plz and clean pyc; remove legacy release workflow; rename crate to kebab-case (openai-client-base)
- release-plz for now
- *(generate)* restrict PR detection and commit to generated artifacts only (src, docs, Cargo.*, stainless.yaml, .openapi-generator)
- *(generate)* prevent pycache from triggering PRs
- *(generate)* disable uv caching to avoid 'No file matched to **/uv.lock' error on GitHub Actions
- *(generate)* install ripgrep for patch script; set OPENAPI_GENERATOR_LOG_LEVEL=error during generation
- default to light clean (remove target/*/build) and add FULL_CARGO_CLEAN=1 opt-in; update docs
- when SKIP_CARGO_CLEAN=1, perform light clean of target/*/build to avoid stale build-script outputs (fixes mime_guess OUT_DIR errors)
- add SKIP_CARGO_CLEAN env var to optionally skip cargo clean during generation
- make openapi-generator log level configurable via OPENAPI_GENERATOR_LOG_LEVEL (default: info); document in PIPELINE.md
- fix generated code to compile cleanly
- Update README and add PIPELINE documentation
- extract shared utilities into utils.py module
- regenerate client with fixed untagged unions
- standardize on uv for all Python operations
- disable release-plz workflow temporarily
- update README with formatting details and generation process
- apply cargo fmt to fix CI formatting check
- update README to document post-processing steps in generation pipeline
- preserve custom files from OpenAPI generator overwrites
- Initial repository setup with OpenAI client generation pipeline
