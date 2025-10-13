# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.0](https://github.com/genai-rs/openai-client-base/compare/v0.6.0...v0.7.0) - 2025-10-13

### Fixed

- add fix_tool_enum_tagging.py to generation pipeline
- prevent fix_untagged_unions from overwriting fix_tool_enum_tagging

### Other

- update generated client from latest OpenAPI spec

## [0.6.0](https://github.com/genai-rs/openai-client-base/compare/v0.5.0...v0.6.0) - 2025-10-12

### Fixed

- Apply enum tagging fixes to 43 model files - fixes tool calling and many other API features by changing conflicting `#[serde(tag = "type")]` to `#[serde(untagged)]`

## [0.5.0](https://github.com/genai-rs/openai-client-base/compare/v0.4.0...v0.5.0) - 2025-10-12

### Fixed

- implement file upload handling for optional binary parameters
- simplify git_release_body template to use changelog variable
- *(renovate)* remove invalid renovateVersion config option
- *(renovate)* pin to v41.124.1 to avoid Cargo artifact update issue
- add rust-version to avoid Renovate Rust 1.90 compatibility issue
- remove custom branchPrefix from renovate config

### Other

- update generated client from latest OpenAPI spec
- Fix tool and union enum serialization conflicts (43 files)
- *(deps)* Update rust minor updates
- *(deps)* Update rust patch updates
- update generated client from latest OpenAPI spec
- update generated client from latest OpenAPI spec
- *(deps)* Update astral-sh/setup-uv action to v7
- Apply cargo fmt to all generated code
- Revert to code patching approach for optional file uploads
- Disable required PR reviews in branch protection
- update generated client from latest OpenAPI spec
- remove decorative emoji from README
- *(deps)* migrate config renovate.json5
- *(deps)* Update mozilla-actions/sccache-action action to v0.0.9
- *(deps)* Update github-actions
- align renovate config with library defaults

## [0.4.0](https://github.com/genai-rs/openai-client-base/compare/v0.3.0...v0.4.0) - 2025-10-02

### Other

- disable PR review requirement
- update generated client from latest OpenAPI spec
- update generated client from latest OpenAPI spec
- install rustfmt/clippy for generator
- update generated client from latest OpenAPI spec
- update generated client from latest OpenAPI spec

## [0.3.0](https://github.com/genai-rs/openai-client-base/compare/v0.2.0...v0.3.0) - 2025-09-18

### Added

- auto derive display for serializable structs
- auto-generate display impls for untagged unions

### Fixed

- regenerate file expiration display impl
- *(workflow)* correct typo 'didname' -> 'name' in security workflow
- populate streaming event enums from inline discriminated unions; correct Assistant/Run/RunStep/Message stream enums; fix ConversationItem derives

### Other

- update generated client from latest OpenAPI spec
- update generated client from latest OpenAPI spec
- *(workflow)* remove .openapi-generator/** from add-paths to prevent PR step failure
- align with langfuse-client-base
- improve workflow
- improve workflow
- make generation idempotent; commit updated untagged content enums and minor model normalizations
- generic fixes for streaming + enums; targeted cleanups for chat content helpers; robust enum derive normalization; fallback serde_json::Value for missing inline anyOf variants; adjust constructors
- re-run generation pipeline; apply generic enum/streaming fixes across models
- add generic enum attribute fixer and integrate into generation pipeline; no more one-off edits

## [0.2.0](https://github.com/genai-rs/openai-client-base/compare/v0.1.3...v0.2.0) - 2025-09-12

### Added

- auto-detect mixed tagged unions (string + object) as untagged enums

### Fixed

- correct acronym casing for mixed union variant types (MCP->Mcp) in untagged union synthesis
- traverse property/array unions and generalize allOf aliasing; repair empty enums and mixed tool_choice union

### Other

- regenerate with mixed-union auto-detection; pipeline builds cleanly
- explicitly set allow_fork_syncing=false on main
- explicitly set lock_branch=false on main
- explicitly block branch creations on main
- explicitly disallow force pushes on main
- explicitly disallow deletions on main
- require conversation resolution on main
- fix topics format and branch protection (min supported), enable admin override via enforce_admins=false
- add job timeout (30m) to avoid indefinite hangs on crates.io wait

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
