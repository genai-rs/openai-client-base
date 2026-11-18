# openai-mock-server

[![crates.io](https://img.shields.io/crates/v/openai-mock-server)](https://crates.io/crates/openai-mock-server)
[![docs.rs](https://img.shields.io/docsrs/openai-mock-server)](https://docs.rs/openai-mock-server)

A scripted mock of the OpenAI API for tests. It serves the API on `127.0.0.1` with
[openai-server-base](https://crates.io/crates/openai-server-base), the server generated from OpenAI's spec, so requests
are decoded and replies encoded with the same types as [openai-client-base](https://crates.io/crates/openai-client-base):
a client that sends what the spec does not describe, or a fixture that does not match the spec, fails the test.

```toml
[dev-dependencies]
openai-mock-server = "0.1"
```

```rust,no_run
use openai_mock_server::{fixtures, op, MockServer, Reply, Stream};
use openai_mock_server::server::chat_api::CreateChatCompletionRequest;

# async fn test() {
let mock = MockServer::start().await;
mock.on(op::CREATE_CHAT_COMPLETION)
    .reply(Reply::json(fixtures::chat_completion("Hello!")))
    .reply(Reply::stream(Stream::new(fixtures::chat_chunks(&["Hel", "lo"]))));

// point the client under test at mock.base_url() (http://127.0.0.1:<port>/v1)

let requests = mock.requests::<CreateChatCompletionRequest>();
assert_eq!(requests[0].body.model, "gpt-4o-mini");
assert!(mock.rejected().is_empty());
# }
```

- **Replies**, queued per operation and taken in order, then the `always` reply: JSON, an event stream, bytes, text, an
  SDP answer, an error in OpenAI's envelope (`Reply::error(404, "...")`), a `429` with `retry-after`
  (`Reply::rate_limited`), with extra headers and a delay.
- **Streams** with the wire details clients meet: keep-alive comments, `event:` names on or off, an error after the
  stream started (chat style `data: {"error": ...}` or Responses style `event: error`), no `[DONE]`, a pause between
  events.
- **Fixtures** for the common bodies: chat completions and tool calls, chunks, responses, function calls and their
  events, embeddings, models, files, batches, images, transcriptions, moderations.
- **Recorded requests** as the generated request structs: the decoded body and the headers. Requests that did not
  decode are listed by `rejected()`.
- **Served:** chat completions, responses, embeddings, models, files, batches, images, audio, moderations and realtime
  calls (`op::SERVED`). Every other operation answers `501`; an operation with no reply scripted too.
- `MockServer::with_api_key` answers `401` to a request without `Authorization: Bearer <key>`.

## License

Licensed under either of [Apache License, Version 2.0](https://github.com/genai-rs/openai-client-base/blob/main/LICENSE-APACHE) or [MIT license](https://github.com/genai-rs/openai-client-base/blob/main/LICENSE-MIT) at your
option.
