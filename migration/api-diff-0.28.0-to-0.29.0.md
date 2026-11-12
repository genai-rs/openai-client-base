# openai-client-base 0.28.0 → 0.29.0: OpenAI spec of 2026-11-12

This release regenerates the client from the OpenAI spec published on 2026-11-12 (`stainless.yaml`, info version
2.3.0). The generator is unchanged: `timvw-rust` 0.24.0 on openapi-generator 7.27.0, and `scripts/generator-config.json`
is unchanged. Every change below comes from the spec.

The machine-readable census is `api-diff-0.28.0-to-0.29.0.json` in this folder (`compare_public_api.py` against main
before the regeneration). It counts 249 added and 9 removed model files, 179 modified ones, and 41 removed enum
variants. The spec-level report (`scripts/api_change_report.py`) lists 28 added operations, none removed, and 5
removed component schemas.

## What changes

### Image output format is a shared enum

The spec now declares `output_format` of image generation and image edits as `$ref: ImagesApiOutputFormat` instead of
an inline enum. The new enum has `Png`, `Webp`, `Jpeg`, `Gif`, `Svg` and `Unknown`.

| Before (0.28) | After (0.29) |
| --- | --- |
| `models::create_image_request::OutputFormat` | `models::ImagesApiOutputFormat` |
| `models::edit_image_body_json_param::OutputFormat` | `models::ImagesApiOutputFormat` |
| `models::images_response::OutputFormat` | `models::ImagesApiOutputFormat` |
| `CreateImageRequest.output_format: Option<Option<create_image_request::OutputFormat>>` | `Option<Option<models::ImagesApiOutputFormat>>` |
| `EditImageBodyJsonParam.output_format: Option<Option<edit_image_body_json_param::OutputFormat>>` | `Option<Option<models::ImagesApiOutputFormat>>` |
| `ImagesResponse.output_format: Option<images_response::OutputFormat>` | `Option<models::ImagesApiOutputFormat>` |
| `images_api::create_image_edit(.., output_format: Option<&str>, ..)` | `output_format: Option<models::ImagesApiOutputFormat>` |
| `images_api::create_image_edit_stream(.., output_format: Option<&str>, ..)` | `output_format: Option<models::ImagesApiOutputFormat>` |

`create_image_edit` and `create_image_edit_stream` also gain the new optional form fields, which shifts the positional
parameters: `frame_count: Option<i64>`, `frame_duration_ms: Option<i64>` and `complexity: Option<models::Complexity>`
after `model`, `markup: Option<std::path::PathBuf>` after `mask`, and `svg: Option<models::SvgParam>` after
`output_format`.

```rust
// 0.28
request.output_format = Some(Some(models::create_image_request::OutputFormat::Png));
// create_image_edit(.., output_format = Some("png"), ..)

// 0.29
request.output_format = Some(Some(models::ImagesApiOutputFormat::Png));
// create_image_edit(.., output_format = Some(models::ImagesApiOutputFormat::Png), svg = None, ..)
```

A string still converts with `"png".parse::<models::ImagesApiOutputFormat>()`.

### Removed models

| Removed | Replacement |
| --- | --- |
| `LiveTransportDtmfSend` (`LiveTransportDTMFSend`) | none; the server event is now `LiveTransportDtmfSent`, and the `LiveServerEvent` / `LiveServerEvent2` variant `LiveTransportDtmfSend` is `LiveTransportDtmfSent` |
| `LiveDecisionTriggerInputParam2` (`LiveDecisionTriggerInputParam-2`) | `LiveDecisionTriggerInputParam` (`LiveSessionCreateParams`, `LiveSessionResourceParam`, `LiveSessionUpdateParams` `.triggers`) |
| `LiveDecisionQuestionsInputParam2` (`LiveDecisionQuestionsInputParam-2`) | `LiveDecisionQuestionsInputParam` (`LiveDecisionTriggerInputParam.decisions`) |
| `LiveDecisionsDelegationParam2` (`LiveDecisionsDelegationParam-2`) | `LiveDecisionsDelegationParam`; the `LiveSessionCreateParamsDelegation` / `LiveSessionResourceParamDelegation` variant is renamed the same way |
| `LiveDecisionsSettingsInputParam2` (`LiveDecisionsSettingsInputParam-2`) | `LiveDecisionsSettingsInputParam` (`LiveDecisionsDelegationParam.decisions`; its `questions` are `Vec<models::LiveDecisionQuestion>`) |
| `LiveDecisionInputItemParamContent`, `LiveDecisionInputItemParamContentOneOfInner` | `LiveDecisionInputItemParam` is now a union (below) |
| `LiveDecisionStoredInputItemParamContent`, `LiveDecisionStoredInputItemParamContentOneOfInner` | `LiveDecisionStoredInputItemParam` is now a union (below) |

### Structs that became unions

| Type | Before | After |
| --- | --- | --- |
| `DecisionInputMessage` | struct with `role: decision_input_message::Role`, `content`, `phase`, `type`; `new(role, content)` | enum `User(DecisionInputUserMessage)`, `Developer(..)`, `System(..)`, `Assistant(..)` |
| `LiveDecisionInputItemParam` | struct with `role`, `content`, `id`, `type`; `new(role, content)` | enum `OneOf(LiveDecisionInputItemParamOneOf)`, `LiveDecisionFunctionCallParam(..)`, `LiveDecisionFunctionCallOutputParam(..)`, `LiveDecisionCustomToolCallParam(..)`, `LiveDecisionCustomToolCallOutputParam(..)`, … |
| `LiveDecisionStoredInputItemParam` | struct with `id`, `role`, `content`, `type`; `new(id, role, content)` | enum `OneOf(LiveDecisionStoredInputItemParamOneOf)`, `LiveDecisionStoredFunctionCallParam(..)`, … |

The nested `Role` / `Type` enums of these three modules are gone. `DecisionInputContentPartsInner` drops the
`OutputText` and `Refusal` variants and gains `InputJson`.

### Changed field and parameter types

| Field / function | Before | After |
| --- | --- | --- |
| `QuestionParamChoice.instructions`, `::new` | `String` | `models::QuestionParamChoiceInstructions` (`Text(String)` or parts) |
| `QuestionParamPredicate.instructions`, `::new` | `String` | `models::QuestionParamPredicateInstructions` |
| `QuestionParamScore.instructions`, `::new` | `String` | `models::QuestionParamScoreInstructions` |
| `LiveDecisionChoiceQuestionParam.instructions`, `::new` | `String` | `models::LiveDecisionChoiceQuestionParamInstructions` |
| `LiveDecisionPredicateQuestionParam.instructions`, `::new` | `String` | `models::LiveDecisionPredicateQuestionParamInstructions` |
| `LiveDecisionScoreQuestionParam.instructions`, `::new` | `String` | `models::LiveDecisionScoreQuestionParamInstructions` |
| `LiveDecisionsSettingsUpdateParam.questions` | `Option<Vec<models::QuestionParam>>` | `Option<Vec<models::LiveDecisionQuestion>>` |
| `RealtimeResponseCreateParams.input` | `Option<Vec<models::RealtimeConversationItem>>` | `Option<Vec<models::RealtimeResponseInputItem>>` |
| `PromptCacheOptions.ttl`, `::new` | `models::PromptCacheTtlEnum` | `prompt_cache_options::Ttl` |
| `ResponsePromptCacheOptionsParam.ttl` | `Option<models::PromptCacheTtlEnum>` | `Option<response_prompt_cache_options_param::Ttl>` |
| `BetaPromptCacheOptions.ttl`, `::new` | `models::BetaPromptCacheTtlEnum` | `beta_prompt_cache_options::Ttl` |
| `BetaResponsePromptCacheOptionsParam.ttl` | `Option<models::BetaPromptCacheTtlEnum>` | `Option<beta_response_prompt_cache_options_param::Ttl>` |
| `BetaResponseUsage.input_tokens_details`, `::new` | `models::ResponseUsageInputTokensDetails` | `models::BetaResponseUsageInputTokensDetails` |
| `BetaResponseUsage.output_tokens_details`, `::new` | `models::ResponseUsageOutputTokensDetails` | `models::BetaResponseUsageOutputTokensDetails` |
| `agents_api::list_agent_sessions` | `(.., agent_id)` | `(.., agent_id, metadata: Option<HashMap<String, String>>)` |
| `agents_api::list_agent_session_events` | `(configuration, session_id)` | `(configuration, session_id, include_obfuscation: Option<bool>)` |

### Removed fields and variants

- `CreateModelResponseProperties.prompt_cache_options` and `BetaCreateModelResponseProperties.prompt_cache_options`
  are gone; the spec keeps `prompt_cache_options` on the request types that use it (`CreateResponse`,
  `CreateChatCompletionRequest`, `ModelResponseProperties`, …).
- `PromptCacheTtlEnum` and `BetaPromptCacheTtlEnum` lose their `Unknown` variant (they are now closed enums with only
  `Variant30m`).
- `CreateanApIkeyforaserviceaccountError` loses `Status409`.

### Added fields

Many existing structs gain optional fields (for example `AgentResource.compaction` / `.experimental`,
`ImagesResponse.svg_usage`). Struct literals must add the new fields; code that uses `::new` or the builder only
breaks where `::new` is listed above.

## Added operations

28 new operations:

- Agent automations (`agents_api`): `create_agent_automation`, `list_agent_automations`, `get_agent_automation`,
  `update_agent_automation`, `delete_agent_automation`, `pause_agent_automation`, `resume_agent_automation`,
  `create_agent_automation_invocation`, `list_agent_automation_invocations`, `get_agent_automation_invocation`.
- Organization external keys (`default_api`): add, list, retrieve, delete and validate an external key config.
- Workload identity providers and their federation rules (`default_api`): create, list, retrieve, update and delete
  (`*_platformworkloadidentityprovider`, `*_platformworkloadidentityfederationrule`).
- Project service tier settings (`default_api`): `retrieveprojectservicetiersettings`,
  `updateprojectservicetiersettings`.
- Audio voices (`audio_api`): `delete_voice`.

Plus the `live_transport_connected_post` webhook and 249 new model files.
