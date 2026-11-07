# openai-client-base 0.24.0 → 0.25.0: generated with openapi-generator 7.27.0

This release generates with openapi-generator 7.27.0 instead of 7.25.0, through the same `timvw-rust` 0.16.0 generator
and the same options. The OpenAI spec is unchanged. The machine-readable census is `api-diff-0.24.0-to-0.25.0.json` in
this folder.

## Why

7.25 lost `null` from a property that a model gets through `allOf`. `ModelResponseProperties.metadata` was already
`Option<Option<HashMap<..>>>`, but `Response.metadata`, the same property reached through `allOf`, was
`Option<HashMap<..>>`. On a few required properties it was worse: `AdminApiKeyCreateResponse.expires_at` is
`anyOf: [integer, null]` and was a bare `i64`, so a key without an expiry failed to decode. 7.27 keeps the `null`, so
every property the spec makes nullable now has the same type wherever it appears.

## What stays the same

- Every model, function, enum and variant keeps its name. No model or function is added or removed.
- `src/apis` keeps every function and signature. JSON request and response bodies are what they were.

## What changes

The census counts 183 field changes in 75 models, 3 constructors and 2 derives:

- **180 optional nullable fields become `Option<Option<T>>`**, with
  `#[serde(default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]`, the way
  484 files already rendered such fields. `None` leaves the field out, `Some(None)` sends or decoded an explicit `null`,
  and `Some(Some(v))` is a value. Most of them: `agent` on 58 Responses resources and stream events
  (`Beta*Resource`, `BetaResponse*Ws*`), `caller` on 8 tool call resources, and the `allOf` properties of
  `CreateResponse`, `Response`, `CreateModelResponseProperties`, `CreateChatCompletionRequest`,
  `ResponsesClientEventResponseCreate` and their `Beta` twins (`metadata`, `temperature`, `top_p`, `user`,
  `safety_identifier`, `prompt_cache_key`, `prompt_cache_retention`, `previous_response_id`, `background`,
  `max_tool_calls`, `top_logprobs`, ...), the fields of `CreateEnvironmentParam`, `AdminApiKeyCreateResponse.name` and
  `last_used_at`, and `FineTuneChatCompletionRequestAssistantMessage.refusal`.
- **3 required nullable fields become `Option<T>`**, decoded with `deserialize_with = "Option::deserialize"` (the field
  must be present, `null` is `None`): `AdminApiKeyCreateResponse.expires_at` (was `i64`), and `annotation` on
  `ResponseOutputTextAnnotationWsAdded` and `BetaResponseOutputTextAnnotationWsAdded` (was the bare union). Their
  `new()` takes the `Option`.
- **`Configuration` and `ApiKey` no longer derive `Debug`.** They implement it by hand and print `[REDACTED]` instead of
  the bearer token, OAuth token, basic-auth password and API key. `{:?}` still works.
- `configuration.rs` re-exports `reqwest_middleware::ClientWithMiddleware`.
- **`list_paginated_fine_tuning_jobs` sends `metadata` in deepObject style**, as the spec declares and the API documents:
  `metadata[key]=value`. 0.24.0 sent each entry as `key="value"`, the key without the `metadata[...]` wrapper and the
  value JSON-quoted, which the API does not read as a metadata filter.

## Migrating

- Setting one of the 180 fields: `Some(v)` becomes `Some(Some(v))`. With the builders, `.temperature(Some(0.2))`. Use
  `Some(None)` only to send an explicit `null`.
- Reading one: `response.metadata.flatten()`, or match `Some(Some(v))`. `Some(None)` means the API sent `null`.
- `AdminApiKeyCreateResponse::new` and the two `*OutputTextAnnotationWsAdded::new` take an `Option`; wrap the value in
  `Some`. Read `expires_at` as an `Option<i64>`: `None` means the key does not expire.
- Code that printed a `Configuration` with `{:?}` to see a token gets `[REDACTED]`.
