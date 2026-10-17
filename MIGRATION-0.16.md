# Migrating to `openai-client-base` 0.16

Version 0.16 changes generated Rust field types to represent omitted, explicitly `null`, and concrete values. Sixty fields that were previously optional become nested `Option<Option<T>>`; three previously required field types also become nested options and are called out separately below because their source requiredness changed or could not be confirmed. This is a source-breaking API change. The broader generated API differences are listed at the end of this guide.

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

Three additional field types changed from required values to nested options: `LiveHostedShellToolInputParam.environment`, `LiveMcpToolChoiceParam.name`, and `PromptCacheOptions.mode`. The raw schema input changes `environment` from required/non-null to optional/nullable. The schema keeps `PromptCacheOptions.mode` required, and the inline source for `LiveMcpToolChoiceParam.name` did not map to a standalone schema component in this comparison; those two Rust/source mismatches have unknown origin and need review before release.

`PromptCacheOptions.ttl` also changes from a required enum field to `Option<Ttl>`, while both compared schema inputs still mark it required. The current Rust model accepts omission for `ttl` and `mode`; the contract test records that behavior, but it is unresolved against the schema and must be reviewed before release.

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

Set `choice.finish_reason` after construction when needed. Constructors for the six request models in the field list still exist and initialize nested optional fields to omission. `CreateCompletionRequest::new` now takes a `String` prompt because `CreateCompletionRequestPrompt` was removed. `PromptCacheOptions::new` no longer takes `ttl` and `mode`; both fields are optional in the generated Rust model, although the raw schema still marks them required.

## Other generated API changes since 0.15.0

The current generated source adds 125 exported model types and removes three: `CreateCompletionRequestPrompt`, `CreateVoicePromptRequest`, and `RealtimeSessionCreateRequestGaTracing`. Code that names one of the removed types must migrate to the current representation (`CreateCompletionRequest.prompt` accepts a string; voice creation now takes `name`, `audio_sample`, and `consent`; realtime tracing uses `TracingConfiguration1`).

New API operations include agent environment create/list, decision create, four agent environment webhooks, project API key update, and vault update. Existing signatures also changed for `list_agent_session_artifacts`, `list_agent_sessions`, `create_voice`, `list_vault_credentials`, and `list_vaults`. Other response/request model field changes are recorded in the declaration and schema provenance reports; their causes are unresolved where the raw schema does not directly explain the generated source difference.

The declaration census and its reproduction script are in [`migration/api-diff-0.15.0-to-0.16.0.json`](migration/api-diff-0.15.0-to-0.16.0.json) and [`migration/compare_public_api.py`](migration/compare_public_api.py). Schema and requiredness evidence plus its Ruby reproduction script are in [`migration/provenance-0.15.0-to-0.16.0.json`](migration/provenance-0.15.0-to-0.16.0.json) and [`migration/compare_schema_provenance.rb`](migration/compare_schema_provenance.rb). Recreate the declaration census with `python3 migration/compare_public_api.py REGISTRY_SRC src`; recreate schema evidence by running `git show 4a996ba65cffd311faed1994cdd78bc73b9007c1:stainless.yaml > REGISTRY_STAINLESS.yaml`, then `ruby migration/compare_schema_provenance.rb REGISTRY_STAINLESS.yaml stainless.yaml migration/api-diff-0.15.0-to-0.16.0.json`. The Rust census compares the verified registry archive (SHA-256 `9f9fc44172785387f606bebb0ab4461d1eec0dfcd4074b0a51791176e9dac0d1`) with source at base commit `7918bf2949d38e201f05ee38729aa9286773a6cc`. It counts declarations, fields, function signatures, enum variant names, derive changes, and serde-field-attribute changes; it does not exhaust macro-generated builders, trait implementations, or all Rust visibility/re-export behavior and is not a complete semver proof. Derive changes include the added `Default` derive on `CreateImageRequest`; the derive census does not expand the generated builder surface. Rustdoc JSON extraction was stopped after 5m49s at 11.7 GB resident memory without output.

## Provenance and unresolved causes

The registry archive records VCS revision `4a996ba65cffd311faed1994cdd78bc73b9007c1`. Comparing its `stainless.yaml` with the candidate base at `7918bf2949d38e201f05ee38729aa9286773a6cc` shows schema input changes. For 53 of the 60 optional-to-nested field transitions that map directly to component properties, both schema versions permit `null`, while the encoding changes from the legacy `nullable: true` form to a type union or `anyOf` including `null`. The generated Rust representation changes from `Option<T>` to `Option<Option<T>>`, which now retains explicit null. Three transitions match unchanged schema references, and four did not map directly; their causes remain unknown.

Of the three previously required-to-nested changes, only `LiveHostedShellToolInputParam.environment` is supported by a matching raw-schema requiredness/nullability change. `PromptCacheOptions.mode` remains required in both raw schema revisions, `PromptCacheOptions.ttl` also remains schema-required despite its Rust field becoming optional, and `LiveMcpToolChoiceParam.name` did not map to a standalone schema component. These are unresolved source/schema mismatches. The three required-to-nested changes and the separate `ttl` change are not attributed to upstream schema evolution without further evidence.

The relevant nullable-field scripts and `scripts/generate.sh` are unchanged between the registry VCS revision and the candidate base. `scripts/fix_untagged_unions.py` did change its string-enum identifier sanitizer; that may explain enum identifier differences, so enum changes are not attributed solely to the OpenAPI input. The PR does not change generated source or generator patches. The new `Default` derive on `CreateImageRequest` is observed in generated source, but its exact generator/schema/patch cause is unknown.

The census records a new `Default` derive for `CreateImageRequest`, alongside the unchanged `bon::Builder` derive. Its exact cause is unknown from the compared source and schema evidence. Other field changes without a direct schema match are also explicitly unresolved in the provenance report. No source-level census here proves that every generated difference is upstream-driven.

## Compatibility

The package manifest is prepared at 0.16.0 because these public changes break source compatibility. Keep consumers on 0.15 if they need the former Rust type signatures. The null-state type changes add no enum values or inferred defaults.
