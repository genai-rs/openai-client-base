# openai-client-base

[![CI](https://github.com/genai-rs/openai-client-base/workflows/CI/badge.svg)](https://github.com/genai-rs/openai-client-base/actions)
[![Security](https://github.com/genai-rs/openai-client-base/workflows/Security/badge.svg)](https://github.com/genai-rs/openai-client-base/actions)
[![MSRV](https://img.shields.io/badge/MSRV-1.82.0-blue)](https://releases.rs/docs/1.82.0/)
[![License](https://img.shields.io/github/license/genai-rs/openai-client-base)](./LICENSE-MIT)

Auto-generated Rust client for the [OpenAI](https://openai.com) API, based on the Stainless OpenAPI specification.

> [!WARNING]
> **🚀 This is a low-level, auto-generated client library.**
> 
> This crate provides raw bindings to the OpenAI API, automatically generated from the OpenAPI specification.
> It is intended as a foundation for building higher-level, more ergonomic client libraries.
>
> **Most users should consider using a higher-level wrapper that provides:**
> - ✅ Simplified API with builder patterns
> - ✅ Better error handling and retries
> - ✅ Automatic rate limiting
> - ✅ More idiomatic Rust interfaces
>
> **Only use this crate directly if you need:**
> - Raw access to all OpenAPI endpoints
> - Custom retry/batching logic
> - Minimal dependencies
> - Full control over API interactions

## Overview

This crate provides the foundational types and API client implementation for OpenAI's API, generated directly from their OpenAPI specification. It's designed to be used as a base dependency for higher-level OpenAI client libraries.

## Features

- 🤖 **Complete API Coverage**: All OpenAI endpoints from the official spec
- 📦 **Auto-generated**: Always up-to-date with the latest API changes
- 🦀 **Pure Rust**: Type-safe bindings with serde serialization
- ⚡ **Async/Await**: Full async support with tokio and reqwest
- 🔄 **Streaming Support**: Server-sent events for compatible endpoints
- 🛠️ **Customizable**: Use as a foundation for your own client

## Generation Pipeline

The client is generated through a comprehensive automated pipeline using Stainless as the single authoritative source:

1. **Fetch Specification** (`fetch_spec.sh`): Download the latest OpenAPI spec from Stainless
   - Source: `https://app.stainless.com/api/spec/documented/openai/openapi.documented.yml`
   - Contains complete definitions for all API endpoints

2. **Apply Spec Patches**:
   - **Layer 1**: Fix model field types and handle allOf inheritance (`fix_model_fields.py`)
   - **Layer 2**: Apply Rust compatibility patches (`patch_spec_rust_compat.py`)

3. **Generate Rust Code**: Use OpenAPI Generator via Docker with reqwest library

4. **Post-Generation Fixes**:
   - Fix module paths and add bon builder support (`patch_generated.rs.sh`)
   - Fix invalid enum variant names (e.g., Gpt4.1 → Gpt4_1)
   - Add Display implementations for multipart types
   - Fix empty enums with proper variants from spec
   - Handle untagged unions (automatically detected from anyOf/oneOf)
   - Fix nullable fields and constructor signatures
   - Manage Default trait based on field types
   - Apply clippy fixes and format code

The pipeline automatically detects and fixes issues rather than hardcoding solutions, making it robust against API changes.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
openai-client-base = { git = "https://github.com/genai-rs/openai-client-base" }
```

## Regenerating the Client

To regenerate the client with the latest OpenAPI spec:

```bash
# Regenerate with latest spec (default behavior)
./scripts/generate.sh

# Or use cached spec during development
USE_CACHED_SPEC=1 ./scripts/generate.sh
```

The generation script automatically:
- Downloads the latest OpenAPI spec from Stainless
- Applies spec-level patches for Rust compatibility
- Generates Rust code using OpenAPI Generator
- Fixes compilation issues (untagged unions, nullable fields, enum variants)
- Manages trait implementations (Default, Display)
- Formats code with `cargo fmt` for consistency

For detailed pipeline documentation, see [PIPELINE.md](PIPELINE.md)

## Requirements

- Rust 1.82+ (stable toolchain recommended for consistent formatting)
- Docker (for OpenAPI Generator)
- `uv` (for Python dependency management) - [Install from here](https://docs.astral.sh/uv/getting-started/installation/)
- ripgrep (`rg`) for post-processing scripts

## License

Licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Contributing

This is an automatically generated crate. For fixes to the generated code, please submit patches to the generation pipeline rather than the generated code directly.
