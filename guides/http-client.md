# HTTP client

`Configuration.client` is a [`timvw_rust_runtime::Client`](https://docs.rs/timvw-rust-runtime): a
[tower](https://docs.rs/tower) service that sends every request. This page covers what it does by default and how to
change it; the [runtime's guide](https://docs.rs/timvw-rust-runtime) has the details.

## Retries

`Configuration::new()` sends with reqwest and retries `429`, `408`, `5xx` and failed connections (Cargo feature
`retry`, on by default), honouring `Retry-After` and backing off with jitter otherwise. An upload is re-sent on retry
(a file is opened again). Without the `retry` feature, or with a client of your own, nothing is retried.

## Timeouts, logging and middleware

Timeouts, logging (Cargo feature `tracing`), extra headers, rate limits and your own middleware are tower layers on
the client:

```rust
use std::time::Duration;
use openai_client_base::apis::configuration::Configuration;

let mut configuration = Configuration::new();
configuration.client = configuration
    .client
    .clone()
    .layer(timvw_rust_runtime::trace::TraceLayer::new())
    .layer(timvw_rust_runtime::timeout::TimeoutLayer::new(Duration::from_secs(120)));
```

`Client::from_service` replaces reqwest with any tower service (a test double, another HTTP stack), and
`Client::from_reqwest` takes a `reqwest::Client` you configured yourself.

## Per-call options

Every operation takes optional `.options(&RequestOptions)`, applied after everything the generated code sets:

- a header replaces the request's values under that name (authentication included);
- query parameters are appended;
- `base_url` sends this one call to another base URL, keeping the operation's path and query.

```rust
use openai_client_base::apis::{models_api, RequestOptions};

let options = RequestOptions::new()
    .header("OpenAI-Project", "proj_123")?
    .base_url("https://eu.api.openai.com/v1")?;
let models = models_api::list_models()
    .configuration(&configuration)
    .options(&options)
    .call()
    .await?;
```

## Other providers

An OpenAI-compatible provider is either a different `base_path` (and credentials), or a
[`ProviderLayer`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/provider/struct.ProviderLayer.html)
on the client, which moves requests to the provider's base URL and adds its headers, query parameters (such as
`api-version`) and path changes:

```rust
use timvw_rust_runtime::provider::ProviderLayer;

let azure = ProviderLayer::new("https://my-resource.openai.azure.com/openai")?
    .rewrite_from("https://api.openai.com/v1")?
    .sensitive_header("api-key", std::env::var("AZURE_OPENAI_API_KEY")?)?
    .query(&[("api-version", "2024-10-21")])?
    .map_path(|path| format!("/deployments/my-deployment{path}"));
configuration.client = configuration.client.clone().layer(azure);
```

Per-call options win over the layer: it does not replace a header or query parameter the options set, and it does not
move a call that has a base URL of its own.

## TLS

The default client is reqwest without default features: choose a TLS backend with this crate's `rustls` (the
default) or `native-tls` feature. Without one, HTTPS requests fail at runtime unless you set a client of your own. See
[Cargo features](cargo-features.md#tls).

## Sending into types of your own

`configuration.client.request(method, url)` builds any request with the same stack, and a response decodes into types
of your own (`response.json::<T>()`, `response.events().json::<T>()` for an event stream). See the
[runtime's guide](https://docs.rs/timvw-rust-runtime).
