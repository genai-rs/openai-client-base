# Upgrading

Releases that change the generated public API bump the minor version. Each one has a
migration note in [`migration/`](../migration), next to the census of what changed in the public API
(`api-diff-<from>-to-<to>.json`, written by `scripts/check_public_api.sh`). The [CHANGELOG](../CHANGELOG.md) lists
every release.

- **0.35** (timvw-rust 0.30.1, runtime 0.4): additive. Every operation with a response body gains an
  `<operation>_response` function that returns an `ApiResponse<T>` with the status, the headers and the body, and the
  spec's declared headers are typed (`<Operation>ErrorHeaders`). See
  [`api-diff-0.34.0-to-0.35.0.md`](../migration/api-diff-0.34.0-to-0.35.0.md).
- **0.34** (timvw-rust 0.30.1, runtime 0.4): `CreateResponse.input` is an `InputParam` (text or items),
  `Response.instructions` is an optional text-or-items union that decodes `null`, and `CreateCompletionRequest.prompt`
  and `CreateModerationRequest.input` are their union types. `logprobs`, `cache_write_tokens` and `access_programs`
  are optional, since the API leaves them out, and 46 input-item enums gain `Unknown`. See
  [`api-diff-0.33.0-to-0.34.0.md`](../migration/api-diff-0.33.0-to-0.34.0.md).
- **0.33** (timvw-rust 0.29.0, runtime 0.4): the multipart unions with a file variant hold `Upload`s (build them with
  `file(..)`/`files(..)`, or pass a `PathBuf` to the builder), `CreateContainerFileBody.file` is an `Upload`,
  `apis::Error::Io` is removed, and `RequestOptions::base_url` moves one call to another base URL. See
  [`api-diff-0.32.0-to-0.33.0.md`](../migration/api-diff-0.32.0-to-0.33.0.md).
- **0.32** (timvw-rust 0.27.0, runtime 0.3): file parameters are `timvw_rust_runtime::Upload` (a `PathBuf` still works;
  bytes and streams too, streamed with a known length); WebAssembly; a missing file is `Error::Http(e)` with
  `e.is_io()`. See [`api-diff-0.31.0-to-0.32.0.md`](../migration/api-diff-0.31.0-to-0.32.0.md).
- **0.31** (timvw-rust 0.26.0, runtime 0.2): credentials are `SecretString`s (`Some(key.into())`); every operation
  takes optional per-call `.options(&RequestOptions)`; Cargo features per API group (`responses`, `audio`, …, with
  `-types` twins; `full` is the default); jittered retries that honour `Retry-After`. See
  [`api-diff-0.30.0-to-0.31.0.md`](../migration/api-diff-0.30.0-to-0.31.0.md).
- **0.30** (timvw-rust 0.25.0): requests go through `timvw-rust-runtime` (a tower service, reqwest by default, with
  retries) instead of `reqwest-middleware`; `Configuration.client` is a `timvw_rust_runtime::Client`, and
  `Error::Reqwest`/`Error::ReqwestMiddleware` become `Error::Http`. Models and operations are unchanged. See
  [`api-diff-0.29.0-to-0.30.0.md`](../migration/api-diff-0.29.0-to-0.30.0.md).
- **0.29** (OpenAI spec of 2026-11-12): image `output_format` is `models::ImagesApiOutputFormat` (replacing
  `create_image_request::OutputFormat` and the other inline `OutputFormat` enums, and the `&str` parameter of
  `images_api::create_image_edit`/`create_image_edit_stream`); `LiveTransportDtmfSend` and the `LiveDecision*Param2`
  models are removed; several decision and realtime types change shape. See
  [`api-diff-0.28.0-to-0.29.0.md`](../migration/api-diff-0.28.0-to-0.29.0.md).
- **0.28** (timvw-rust 0.22.0): `organization_decisionsusage` takes `bucket_width` as
  `models::DecisionsUsageBucketWidth` instead of `&str`, and the values of `ChatkitWorkflow.state_variables` are
  `Option<ChatkitWorkflowStateVariablesValue>` (a `null` value now decodes). See
  [`api-diff-0.27.0-to-0.28.0.md`](../migration/api-diff-0.27.0-to-0.28.0.md).
- Older releases: the notes in [`migration/`](../migration), and [`MIGRATION-0.16.md`](../MIGRATION-0.16.md) for the
  move to 0.16.
