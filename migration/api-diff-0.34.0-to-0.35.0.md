# openai-client-base 0.34 → 0.35.0: status and headers per call

This release turns on the generator's `responseFunctions` option (`timvw-rust` 0.30.1, runtime 0.4, the same OpenAI
spec as 0.34). The census is `api-diff-0.34.0-to-0.35.0.json`. Every change adds API; nothing is removed or changes
type, so code written for 0.34 compiles unchanged.

## What is added

- **`<operation>_response` functions** (394, one per operation with a response body). Each sends the same request as
  its operation and fails the same way, but returns an `apis::ApiResponse<T>` instead of `T`.
- **`apis::ApiResponse<T>`**: the success `status`, its `headers` (for example `x-ratelimit-*` and `x-request-id`) and
  the body undecoded. `json()` decodes it as `T`, `json_as::<U>()` and `json_borrowed::<U>()` as a type of yours,
  `text()` returns it as it arrived, and `typed_headers::<H>()` reads the declared headers.
- **Typed headers** where the spec declares response headers, in the operation's API module:
  - `<Operation>ErrorHeaders` (179), mostly `retry_after: Option<i64>`, built from the headers of an
    `Error::ResponseError` with `from_headers(&response.headers)`;
  - `CreateRealtimeCallResponseHeaders` (`location`), read from an `ApiResponse` with `typed_headers()`.

  Each field is `None` when the header is missing or does not parse.

## Upgrading

Nothing to change. To read the rate-limit headers of a call, use its `_response` function:

<!-- not compiled: needs a configuration and a request -->
```rust
let response = responses_api::create_response_response(&configuration, &request, None).await?;
let remaining = response.headers.get("x-ratelimit-remaining-requests");
let body: models::Response = response.json()?;
```

See [Status and headers](../guides/usage.md#status-and-headers).
