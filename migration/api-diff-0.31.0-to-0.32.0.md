# openai-client-base 0.31.0 → 0.32.0: timvw-rust 0.27.0, runtime 0.3

This release regenerates the client from the same spec with `timvw-rust` 0.27.0 (openapi-generator 7.27.0) and moves
to `timvw-rust-runtime` 0.3. The census is `api-diff-0.31.0-to-0.32.0.json`: no model or field changes; the operations
that upload files change their parameter type.

## What changes

**File parameters are `timvw_rust_runtime::Upload` (breaking for code that names the type).** The builders take
`impl Into<Upload>`, so passing a `PathBuf` still compiles; you can now also pass bytes, a stream or a reader:

```rust
use timvw_rust_runtime::Upload;
// a file, streamed from disk (and re-opened if the request is retried)
files_api::create_file().configuration(&configuration).file(PathBuf::from("data.jsonl")).purpose(purpose).call().await?;
// bytes already in memory
files_api::create_file()
    .configuration(&configuration)
    .file(Upload::bytes(jsonl).file_name("data.jsonl").mime("application/jsonl"))
    .purpose(purpose)
    .call()
    .await?;
```

Files are no longer read into memory with a blocking `std::fs::read`: they are streamed with a known `Content-Length`,
and a part's `Content-Type` is guessed from the file name's extension (`image/png`, `video/mp4`, ...). The multipart
union models with a file variant still hold a `PathBuf`.

**A missing file is `Error::Http(e)` with `e.is_io()`** (it was `Error::Io`).

**WebAssembly.** The crate builds for `wasm32-unknown-unknown`, e.g.
`default-features = false, features = ["responses"]` (the browser does TLS; files from paths are not available, bytes
and streams are). Retries back off with browser timers.

**Runtime 0.3** adds `ProviderLayer` (OpenAI-compatible providers), `Retry-After` HTTP dates, `save_to`/`into_async_read`
for downloads and an idle timeout for event streams (`EventStream::idle_timeout`). The crate no longer depends on
`tokio`/`tokio-util` directly.

## Upgrading

- Code that named `PathBuf` as an operation's parameter type uses `Upload` (or keeps passing a `PathBuf` to the builder).
- `match` arms on `Error::Io` for a missing upload file match `Error::Http(e) if e.is_io()`.
