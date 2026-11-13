# Cargo features

The default features are `rustls`, `retry` and `full`: every API, with rustls for TLS and retries in the default
client.

## API groups

Every API group is a Cargo feature, named after async-openai's: `responses`, `chat-completion`, `audio`, `image`,
`embedding`, `file`, `vectorstore`, `administration`, `evals`, `realtime`, `video`, `container`, … The groups, and the
tags and paths each one holds, are in [`scripts/api-features.yaml`](../scripts/api-features.yaml), the generator's
`apiFeaturesFile`.

- `<group>` compiles the group's API functions, its models and the HTTP client.
- `<group>-types` compiles only the group's models, without the HTTP client. The groups whose models hold an upload
  (`container-types`, `skill-types`, `video-types`) also pull in `timvw-rust-runtime` for its `Upload` type.
- `types` is every model, `full` everything.

```toml
[dependencies]
# only the Responses API (and its models)
openai-client-base = { version = "0.x", default-features = false, features = ["rustls", "retry", "responses"] }
# only the Responses models, no HTTP client
openai-client-base = { version = "0.x", default-features = false, features = ["responses-types"] }
```

Use the current version from the crates.io badge on the [README](../README.md) (or `cargo add openai-client-base
--no-default-features --features rustls,retry,responses`).

## TLS

The default client is reqwest (through `timvw-rust-runtime`) without its default features, so a TLS backend is chosen
through this crate's features; without one, HTTPS requests fail at runtime unless you set a client of your own.

- `rustls` (default): pure Rust.
- `native-tls`: the platform's TLS (OpenSSL on Linux, Schannel on Windows, Security Framework on macOS).

```toml
openai-client-base = { version = "0.x", default-features = false, features = ["native-tls", "retry", "full"] }
```

## Other features

- `retry` (default): the default client retries `429`, `408`, `5xx` and failed connections (see
  [HTTP client](http-client.md#retries)).
- `tracing`: `timvw_rust_runtime::trace::TraceLayer`, which logs each request and retry with `tracing`.

## WebAssembly

The crate builds for `wasm32-unknown-unknown` with `default-features = false` and the groups you need (the browser
does TLS, so no TLS feature):

```toml
openai-client-base = { version = "0.x", default-features = false, features = ["responses"] }
```

Uploads from paths are not available there; bytes and streams are. Retries back off with browser timers.

## Minimum supported Rust version

The MSRV is the one of `timvw-rust-runtime` (see the MSRV badge). To build on it, resolve dependency versions that
support it: `CARGO_RESOLVER_INCOMPATIBLE_RUST_VERSIONS=fallback cargo update` (or `resolver = "3"` in your workspace).
