# openai-client-base 0.29.0 → 0.30.0: timvw-rust 0.25.0, a tower HTTP stack

This release regenerates the client from the same spec (`openapi.json` unchanged) with `timvw-rust` 0.25.0 on
openapi-generator 7.27.0. `scripts/generator-config.json` is unchanged. The models and the operations' signatures do
not change; what changes is how requests are sent.

## What changes

The crate depends on [`timvw-rust-runtime`](https://docs.rs/timvw-rust-runtime) instead of `reqwest` and
`reqwest-middleware`. Requests go through a [tower](https://docs.rs/tower) service, reqwest by default, with retries.

| Before (0.29) | After (0.30) |
| --- | --- |
| `Configuration.client: reqwest_middleware::ClientWithMiddleware` | `Configuration.client: timvw_rust_runtime::Client` |
| `apis::configuration::ClientWithMiddleware` (re-export) | `apis::configuration::Client` |
| `apis::Error::Reqwest(reqwest::Error)`, `apis::Error::ReqwestMiddleware(reqwest_middleware::Error)` | `apis::Error::Http(timvw_rust_runtime::Error)` |
| `ResponseContent.status: reqwest::StatusCode`, `.headers: reqwest::header::HeaderMap` | `timvw_rust_runtime::StatusCode`, `timvw_rust_runtime::HeaderMap` (the same `http` types) |
| `sse::Events::from_bytes` over `Result<_, reqwest::Error>` chunks | over `Result<_, timvw_rust_runtime::Error>` chunks |
| a file or stream response is a `reqwest::Response` | a `timvw_rust_runtime::Response` (`status()`, `headers()`, `bytes()`, `bytes_stream()`, `events()`) |
| features `rustls` (default), `native-tls` | the same, plus `retry` (default) and `tracing` |
| no retries unless added with `reqwest-retry` | `Configuration::new()` retries `429`, `408`, `5xx` and failed connections (3 times, `Retry-After`) |

## Upgrading

- A configured `reqwest::Client` (proxy, timeouts): `Client::from_reqwest(reqwest_client)`, which keeps the default
  retries, or `Client::from_service(timvw_rust_runtime::ReqwestService::new(reqwest_client))` without them.
- A `reqwest_middleware` middleware becomes a tower layer: `configuration.client = configuration.client.clone().layer(..)`.
  `reqwest-retry` is no longer needed; drop it, or the requests are retried twice over.
- A `match` on `Error::Reqwest` or `Error::ReqwestMiddleware` matches `Error::Http(e)`; `e.is_timeout()`,
  `e.is_connect()` and `e.source()` (the `reqwest::Error`) say what failed.

The runtime's [guide](https://docs.rs/timvw-rust-runtime) covers retries, timeouts, logging, middleware and rate limits,
and the generator's
[HTTP stack guide](https://github.com/timvw/openapi-generator-timvw-rust/blob/main/docs/guides/http-stack.md#moving-from-reqwest-middleware-to-tower)
maps reqwest-middleware code to tower.
