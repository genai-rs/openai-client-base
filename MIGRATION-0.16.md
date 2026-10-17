# Migrating to `openai-client-base` 0.16

Version 0.16 changes generated Rust field types to preserve the three states the OpenAI schema permits for selected optional properties: omitted, explicitly `null`, and a concrete value. This is a source-breaking API change. The 60 null-state type changes below are part of a broader generated OpenAI API update; additional additions and removals are listed at the end of this guide.

## Optional fields

The following fields now use `Option<Option<T>>`:

- `CreateChatCompletionRequest`: `top_logprobs`, `max_completion_tokens`, `frequency_penalty`, `presence_penalty`, `store`, `stream`, `stop`, `logprobs`, `max_tokens`, `n`, `prediction`, and `seed`.
- `CreateCompletionRequest`: `best_of`, `echo`, `frequency_penalty`, `logprobs`, `max_tokens`, `n`, `presence_penalty`, `seed`, `stop`, `stream`, `suffix`, `temperature`, and `top_p`.
- `CreateImageRequest`: `n`, `quality`, `response_format`, `output_format`, `output_compression`, `stream`, `size`, `moderation`, `background`, and `style`.
- `CreateRunRequest`: `instructions`, `additional_instructions`, `additional_messages`, `tools`, `temperature`, `top_p`, `stream`, `max_prompt_tokens`, and `max_completion_tokens`.
- `UpdateVectorStoreRequest`: `name`.
- `CreateFineTuningJobRequest`: `suffix`, `validation_file`, `integrations`, and `seed`.
- `CreateFineTuningJobRequestIntegrationsInnerWandb`: `name` and `entity`.
- `CreateThreadAndRunRequest`: `instructions`, `tools`, `temperature`, `top_p`, `stream`, `max_prompt_tokens`, and `max_completion_tokens`.
- `RealtimeSessionCreateRequestGa`: `tracing`.

Use the outer option to choose whether to send a property. `None` omits it, `Some(None)` sends JSON `null`, and `Some(Some(value))` sends the value. For example:

```rust
request.store = None;                    // omit `store`
request.store = Some(None);              // send `"store": null`
request.store = Some(Some(false));       // send `"store": false`
request.max_completion_tokens = Some(Some(0));
```

The same distinction applies to empty strings, empty arrays, and numeric zero. Keep existing `None` assignments when omission is intended. Wrap a value in both options when you intend to send it. Use `Some(None)` only when the API should receive explicit `null`.

In 0.15, these fields used `Option<T>`, so a Rust value of `None` serialized as omission and the type could not represent explicit `null`. Code that constructs or pattern-matches these fields must be updated. Serde now also accepts a missing property for every affected field while retaining an explicit null on deserialization and serialization.

## Streamed chat choice finish reason

`CreateChatCompletionStreamResponseChoicesInner.finish_reason` is now `Option<Option<FinishReason>>`. This accepts streamed choices where `finish_reason` is absent, including the final audio update containing only `delta.audio.expires_at`. Omission is `None`, an explicit JSON null is `Some(None)`, and a reason is `Some(Some(reason))`. The existing five wire variants are unchanged: `stop`, `length`, `tool_calls`, `content_filter`, and deprecated `function_call`.

The constructor no longer takes `finish_reason` because it is optional:

```rust
let choice = CreateChatCompletionStreamResponseChoicesInner::new(delta, index);
```

Set `choice.finish_reason` after construction when needed. Required arguments for the six changed request model constructors are unchanged; each constructor initializes the newly nested optional fields to omission.

## Other generated API changes since 0.15.0

The OpenAI schema update also adds 125 exported model types and removes three: `CreateCompletionRequestPrompt`, `CreateVoicePromptRequest`, and `RealtimeSessionCreateRequestGaTracing`. Code that names one of the removed types must migrate to the current generated representation (`CreateCompletionRequest.prompt` accepts a string; voice creation now takes `name`, `audio_sample`, and `consent`; realtime tracing uses `TracingConfiguration1`).

New API operations include agent environment create/list, decision create, four agent environment webhooks, project API key update, and vault update. Existing signatures also changed for `list_agent_session_artifacts`, `list_agent_sessions`, `create_voice`, `list_vault_credentials`, and `list_vaults`. Several response/request model fields changed type as the upstream schema evolved; review the generated model definitions when migrating code that constructs those models.

The source-level public API inventory is in [`docs/api-diff-0.15.0-to-0.16.0.json`](docs/api-diff-0.15.0-to-0.16.0.json). It compares the verified `openai-client-base` 0.15.0 registry archive (SHA-256 `9f9fc44172785387f606bebb0ab4461d1eec0dfcd4074b0a51791176e9dac0d1`) with the current generated source at base commit `7918bf2949d38e201f05ee38729aa9286773a6cc`. It records added/removed model exports, public field and function signature changes, and enum variant changes. Rustdoc JSON extraction could not complete within 5m49s and reached 11.7 GB resident memory, so the checked-in inventory is derived from the public Rust source declarations.

## Compatibility

The package manifest is prepared at 0.16.0 because these public changes break source compatibility. Keep consumers on 0.15 if they need the former Rust type signatures. The null-state changes add no enum values or inferred defaults.
