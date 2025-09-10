# openai-client-base

[![CI](https://github.com/genai-rs/openai-client-base/workflows/CI/badge.svg)](https://github.com/genai-rs/openai-client-base/actions)
[![Security](https://github.com/genai-rs/openai-client-base/workflows/Security/badge.svg)](https://github.com/genai-rs/openai-client-base/actions)
[![MSRV](https://img.shields.io/badge/MSRV-1.82.0-blue)](https://releases.rs/docs/1.82.0/)
[![License](https://img.shields.io/crates/l/openai-client-base)](./LICENSE-MIT)

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

The client is generated through a streamlined pipeline using Stainless as the single authoritative source:

1. **Fetch**: Download the Stainless OpenAPI spec (authoritative source)
   - Source: `https://app.stainless.com/api/spec/documented/openai/openapi.documented.yml`
   - Contains complete definitions for all API endpoints
2. **Fix Models**: Handle allOf inheritance and field type resolution
3. **Rust Patches**: Add experimental fields as `Option<T>` and apply compatibility fixes  
4. **Generate**: Use OpenAPI Generator to create Rust code
5. **Post-process**: Apply final code patches:
   - Fix empty enums for discriminated unions
   - Automatically convert acronyms to Rust naming conventions (MCP → Mcp, HTTP → Http, etc.)
   - Remove Default implementations from empty enums

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
- Fixes model inheritance and field types
- Generates Rust code using OpenAPI Generator
- Applies post-processing fixes (empty enums, acronym casing)
- Formats code with `cargo fmt` for consistency

## Requirements

- Rust 1.82+ (stable toolchain recommended for consistent formatting)
- Docker (for OpenAPI Generator)
- Python 3.8+ with PyYAML (or `uv` tool)

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