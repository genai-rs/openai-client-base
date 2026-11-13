# openai-client-base 0.30.0 → 0.31.0: timvw-rust 0.26.0, runtime 0.2

This release regenerates the client from the same spec (`openapi.json` unchanged) with `timvw-rust` 0.26.0 on
openapi-generator 7.27.0 and moves to `timvw-rust-runtime` 0.2. The census is `api-diff-0.30.0-to-0.31.0.json`
(`compare_public_api.py` against v0.30.0): no model or field is added or removed; every operation gets one optional
argument, and three `Configuration` fields change type.

## What changes

**Credentials are `SecretString`s (breaking).** `bearer_access_token` and `oauth_access_token` are
`Option<SecretString>`, the password of `basic_auth` is `Option<SecretString>` and `ApiKey::key` is a `SecretString`
(`secrecy`, re-exported as `apis::configuration::{SecretString, ExposeSecret}`). `Debug` stays redacted and the
`Authorization` header is marked sensitive.

```rust
// 0.30
configuration.bearer_access_token = Some(std::env::var("OPENAI_API_KEY")?);
// 0.31
configuration.bearer_access_token = Some(std::env::var("OPENAI_API_KEY")?.into());
```

Code that read the token back calls `token.expose_secret()`.

**Per-call request options (additive).** Every operation's builder takes `.options(&options)`, a
`timvw_rust_runtime::RequestOptions` (re-exported as `apis::RequestOptions`) with extra headers and query parameters,
applied after everything the operation sets. A call that does not set it sends what it sent before.

```rust
let options = openai_client_base::apis::RequestOptions::new().header("OpenAI-Beta", "assistants=v2")?;
responses_api::create_response().configuration(&configuration).create_response(&request).options(&options).call().await?;
```

**Cargo features per API group (additive, default unchanged).** The crate has async-openai's feature groups
(`responses`, `chat-completion`, `audio`, `administration`, … — see `scripts/api-features.yaml`), each with a
`-types` twin that compiles only the group's models, without the HTTP client. `full` (the default) is everything, so
existing users see no change; `default-features = false, features = ["rustls", "responses"]` compiles the Responses API
alone (~19 s instead of ~77 s for this crate in a dev build). `apis` needs at least one API feature.

**Retries (runtime 0.2).** The default client still retries `429`, `408`, `5xx` and failed connections 3 times; the
backoff is now jittered, `Retry-After` (and `retry-after-ms`) is honoured up to 60 s instead of being capped at 8 s,
and a `Retry-After` above 60 s is returned at once. `timvw_rust_runtime::retry::RetryLayer` with
`.permanent_429(timvw_rust_runtime::retry::openai_insufficient_quota)` leaves `insufficient_quota` 429s unretried
(opt-in; openai-ergonomic enables it).

## Upgrading

- Add `.into()` (or `SecretString::from`) where you set a credential.
- A lock file that holds `timvw-rust-runtime` 0.1 updates with `cargo update -p timvw-rust-runtime`.
- With `default-features = false`, add `full` or the groups you use.
