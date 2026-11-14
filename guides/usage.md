# Usage

`openai-client-base` has a module per API tag under `apis` (`responses_api`, `chat_api`, `files_api`, …) with one
async function per operation, and a type per schema under `models`. Every function has a
[bon](https://bon-rs.com) builder: set the parameters, then `.call().await`. The
[API reference](https://docs.rs/openai-client-base) lists them all; the generated
[`docs/`](https://github.com/genai-rs/openai-client-base/tree/main/docs) has a page per API and model.

## Configuration

`Configuration::new()` points at `https://api.openai.com/v1` and sends with the default client (reqwest with
retries). Credentials are `SecretString`s, redacted in `Debug`:

```rust
use openai_client_base::apis::configuration::Configuration;

let mut configuration = Configuration::new();
configuration.bearer_access_token = Some(std::env::var("OPENAI_API_KEY")?.into());
```

Set `base_path` for another endpoint (Azure OpenAI, a proxy, a compatible provider), and `user_agent` to name your
application. One `Configuration` is meant to be shared by every call: cloning it is cheap.

## Calling an operation

```rust
use openai_client_base::apis::models_api;

let models = models_api::list_models()
    .configuration(&configuration)
    .call()
    .await?;
println!("{} models available", models.data.len());
```

Request bodies are models built with `new` (the required fields) and then their public fields, or with their
`builder()`. Optional parameters of the operation are optional setters on its builder.

An operation that takes or returns more than one media type has a function per media type: `create_image_edit`
sends multipart, `create_image_edit_json` JSON.

## Streaming

A `text/event-stream` response is a function returning `apis::sse::EventStream` of the spec's event type:
`create_response_stream` yields `ResponseStreamEvent`s, `create_chat_completion_stream`
`CreateChatCompletionStreamResponse`s. With `futures-util` for `StreamExt`:

```rust
use futures_util::StreamExt;
use openai_client_base::apis::responses_api;
use openai_client_base::models::{CreateResponse, ResponseStreamEvent};

let mut request = CreateResponse::new();
request.model = Some("gpt-5".to_owned());
request.input = Some("Write a haiku about Rust.".to_owned());
request.stream = Some(Some(true));
let mut events = responses_api::create_response_stream()
    .configuration(&configuration)
    .create_response(&request)
    .call()
    .await?;
while let Some(event) = events.next().await {
    if let ResponseStreamEvent::ResponseTextDeltaEvent(delta) = event? {
        print!("{}", delta.delta);
    }
}
```

Each `data:` payload decodes to the event type by its `type` field (a type this client does not know yet is
`ResponseStreamEvent::Unknown`); `data: [DONE]` ends the stream. An error status is the operation's typed error, as
for `create_response`.

## Uploading files

File parameters, the file variants of multipart unions and file fields of models are
[`timvw_rust_runtime::Upload`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.Upload.html)s: a
path, bytes, a reader or a stream. The builders take anything that converts into one, so a `PathBuf` works as is:

```rust
use std::path::PathBuf;
use openai_client_base::apis::{files_api, skills_api, videos_api};
use openai_client_base::models;
use timvw_rust_runtime::Upload;

// a file, streamed from disk (and opened again if the request is retried)
files_api::create_file()
    .configuration(&configuration)
    .file(PathBuf::from("data.jsonl"))
    .purpose("batch")
    .call()
    .await?;

// bytes already in memory, with the file name and type the server should see
files_api::create_file()
    .configuration(&configuration)
    .file(Upload::bytes(jsonl).file_name("data.jsonl").mime("application/jsonl"))
    .purpose("batch")
    .call()
    .await?;

// a multipart union with a file variant: `file(..)`, or `files(..)` for several parts
videos_api::create_video_edit()
    .configuration(&configuration)
    .video(models::CreateVideoEditMultipartBodyVideo::file(PathBuf::from("clip.mp4")))
    .prompt("make it night")
    .call()
    .await?;
skills_api::create_skill()
    .configuration(&configuration)
    .files(models::CreateSkillBodyFiles::files([PathBuf::from("SKILL.md"), PathBuf::from("run.py")]))
    .call()
    .await?;
```

Files are streamed with a known `Content-Length`; a part's `Content-Type` comes from the file name's extension unless
you set one. An upload has no JSON form: the file variants and fields are left out of a model's JSON.

## Status and headers

Every operation with a response body also has a `<operation>_response` function. It returns an `ApiResponse<T>`
with the `status`, the `headers` (for example OpenAI's `x-ratelimit-*` and `x-request-id`) and the body, which
`json()` decodes as the generated type.

<!-- not compiled: needs a configuration and a request -->
```rust
let response = responses_api::create_response_response(&configuration, &request, None).await?;
let remaining = response.headers.get("x-ratelimit-remaining-requests");
let body: models::Response = response.json()?;
```

## Errors

Every operation returns `Result<T, apis::Error<E>>`, where `E` is the operation's typed error (an enum of the error
responses the spec documents, plus `UnknownValue`):

- `Error::ResponseError(response)`: the server answered with an error status. `response.status`, `response.headers`
  (`x-request-id`, `retry-after`, rate limits), the raw `response.content`, and `response.entity` decoded into `E`
  when the body matches.
- `Error::Http(e)`: building, sending or reading the request failed: `e.is_timeout()`, `e.is_connect()`, `e.is_io()`
  (a file to upload could not be read), …
- `Error::Serde(e)`: a success response did not decode into the documented type.

```rust
use openai_client_base::apis::{files_api, Error};

match files_api::retrieve_file().configuration(&configuration).file_id("file-123").call().await {
    Ok(file) => println!("{}", file.filename),
    Err(Error::ResponseError(response)) => eprintln!("{}: {}", response.status, response.content),
    Err(e) => eprintln!("{e}"),
}
```

## Per-call options

Every operation takes optional `.options(&RequestOptions)`: extra headers, query parameters or another base URL for
one call. See [HTTP client](http-client.md#per-call-options).
