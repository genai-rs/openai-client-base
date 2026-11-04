# openai-client-base 0.19.0 → 0.20.0: stable names, unions from the model, decoding by tag

This release generates with `timvw-rust` 0.12.0 and its `qualifiedInlineNames`, `modelUnions` and `unionDispatch`
options (Milestone 3.3 of the [generator design](https://claude.ai/artifact/ABar8Up91gjVMDHgHL1GE3)). The OpenAI spec
is unchanged. The machine-readable census is `api-diff-0.19.0-to-0.20.0.json` in this folder; its `renames` key holds
the maps below.

## What stays the same

- Every operation in `src/apis` keeps its name and parameters. One return type has a new name: `beta_delete_response`
  returns `BetaDeleteResponse200Response`.
- Field names keep their names and their JSON names. 137 fields change type, and in each only the name of a model
  changes (`Option<models::Object0>` becomes `Option<models::AssistantObjectToolResources>`).
- A request has the same values on the wire, apart from the fixes listed below. A union that was internally tagged now
  writes its tag where its struct has the field, so `role` can move within the object.
- What 0.19.0 added stays: `FromStr` on closed enums, and `Default` only where valid.
- Unions keep `Display`, `new_text`, `new_<vec variant>`, `From<String>` and `From<&str>`.

## What changes

### Names come from the schema and cannot collide

- An inline schema keeps its title as its name only when that title is unique. Otherwise it is named after where it
  stands: property `p` of `P` is `PP`, array items `PInner`, map values `PValue`. So `Object0` is now
  `AssistantObjectToolResources`, and no model is a bare `Object<i>` or carries a `_1` collision suffix.
- Two different schemas with the same Rust name now fail generation, naming both. They used to overwrite each other.
- Schemas that only had the same shape no longer share a model. `AuditLog::invite_deleted` was an
  `AuditLogInviteAccepted`, and is now an `AuditLogInviteDeleted`.
- 122 model exports are gone and 206 are new, 123 and 208 files. Of the removed models 82 have a successor (69 one, 13
  several), 31 were string enums nothing used, and 9 have no successor. 66 models keep their name but no longer stand
  in other locations. The lists are at the end of this note.

### Unions render from the model

- Every `oneOf` and `anyOf` is an enum from the spec's branches instead of a text rewrite: 376 unions become 409, none
  internally tagged (10 before), none empty (3 before).
- A variant is named after its branch: discriminator key, then referenced schema, title, then type. 308 variants in
  119 unions have new names, mostly in case (`Chatcompletionrequestmessagecontentparttext` is
  `ChatCompletionRequestMessageContentPartText`, `Vadconfig` is `VadConfig`).
- A `null` branch is no variant any more, so 40 unions lose `Null`. Where the field is nullable it is an `Option`.
- A union of one string enum and `null` is that enum: `BetaMessagePhase`, `BetaReasoningEffort`,
  `BetaServiceTierResponses`, `BetaVerbosity`, `MessagePhase`, `ReasoningEffort`, `ServiceTier`,
  `ServiceTierResponses`, `Verbosity`. `ReasoningEffort::TextVariant(ReasoningEffortTextVariantEnum::High)` is
  `ReasoningEffort::High`.
- 30 variants that held a `serde_json::Value` hold their type, such as
  `RunStreamEvent::Object0(RunStreamEventObject0)`.
- Variants are ordered so that the narrower of two goes first (an integer before a float, a closed enum before a free
  string, more required fields first). An untagged union can therefore pick another variant than before.

### Tagged unions decode by their tag

246 of the 409 unions have a tag, from a `discriminator` or a required property with fixed values. They read the tag
and decode only the variant it selects. They still serialize as before and hold the same structs. The other 163 stay
`#[serde(untagged)]`.

| Input | 0.19.0 | 0.20.0 |
| --- | --- | --- |
| `InputItem`, unknown tag | `data did not match any variant of untagged enum InputItem` | ``InputItem: unknown `type` value "nope"`` |
| `InputItem`, field missing | `data did not match any variant of untagged enum InputItem` | ``InputItem::Item: Item::FunctionToolCall: missing field `name` `` |
| `ChatCompletionRequestMessage`, no tag | ``missing field `role` `` | ``ChatCompletionRequestMessage: tag `role` is missing or null`` |
| `ChatCompletionRequestMessage`, no object | ``invalid type: integer `3`, expected internally tagged enum ...`` | ``ChatCompletionRequestMessage: expected an object with tag `role`, got a number`` |

### Bugs this fixes

- **`ChatCompletionRequestMessage` sent `role` twice and could not decode.**
  `{"role":"user","content":"hi","role":"user"}` went out on every chat request, and decoding any message failed with
  ``missing field `role` ``. The other unions with an internal tag whose structs hold it
  (`ComputerUseApprovalResponseKindResource`, `ComputerUseApprovalResponseParamBrowserAuthentication`,
  `LiveInitialItem`, `UsageTimeBucketResultsInner`) had the same problem.
- **Stack overflow.** Decoding any object as `AuditLogActor`, `BetaMcpToolCallError`, `McpToolCallError`,
  `TracingConfiguration` overflowed the stack and aborted the process, since each held itself as a variant.
  `AuditLogActor` stands in every audit log entry.
- **Null-only enums.** `BetaPrompt`, `BetaResponseStreamOptions`, `ChatCompletionStreamOptions`, `FineTuningJobError`,
  `Prompt`, `ResponseStreamOptions`, `RunCompletionUsage`, `RunStepCompletionUsage` could only hold `null`, and now
  hold their fields. A stream option or a usage block could not be sent or read.
- **Empty enums.** `BetaResponsesServerEvent`, `LiveServerEvent`, `ResponsesServerEvent` had no variants and could not
  decode at all. They hold their events now.
- **Lost or wrong variants.**
  - `CreateTranscription200Response` was one struct with the required fields of all three response forms, so a plain
    `{"text":"..."}` failed on ``missing field `task` ``. It is now a union of the three, which changes how you read
    it, see below.
  - `RealtimeAudioFormats`, `RealtimeTruncation`, `RealtimeTurnDetection`, `ResponseError`,
    `WebSearchApproximateLocation` and others held `ChatCompletionNamedToolChoice` where the spec has another object,
    and rejected the real JSON.
  - Hyperparameter unions such as `FineTuneDpoHyperparametersBatchSize` lost their integer branch.
  - An `InputItem` message with a `status` decoded as `EasyInputMessage`, which has no such field, and lost it. It is
    now an `Item`.
- **Multipart text parts were JSON-quoted.** `create_video`'s `model` went out as `"sora-2"` with the quotes, as did
  `create_image_edit`'s `size`. They are plain text now.
- **File variants in multipart unions did not upload the file.** `create_video`'s `input_reference`, the `video` of
  `create_video_edit` and `create_video_extend`, and the `files` of `create_skill` and `create_skill_version` sent the
  path as JSON-quoted text. A path variant now uploads the file, one part per file for an array, and a missing file is
  `Error::Io`. An object variant goes out as one part with `Content-Type: application/json`.

### Behaviour to know about

- **A repeated key in an object is last-wins under a tagged union.** The union reads a `serde_json::Value` first,
  which keeps the last value, also deeper in the object. `{"type":"apply_patch","type":"shell"}` as a `Tool` is a
  shell tool; it was an error. A struct decoded on its own still rejects a repeated key.
- **Decoding is usually faster, and some `InputItem` messages slower.** Release build on one machine, nanoseconds per
  decode, best of three runs of 100,000, 0.19.0 against 0.20.0:

| Input | 0.19.0 | 0.20.0 |
| --- | --- | --- |
| `Tool`, `{"type":"apply_patch"}` | 2674 | 136 |
| `AssistantStreamEvent`, `done` | 494 | 216 |
| `InputItem`, 2 KB `function_call` | 1852 | 677 |
| `InputItem`, unknown tag | 5851 | 335 |
| `InputItem`, 2 KB `message` with text content | 791 | 2174 |

  A message used to match the first variant, `EasyInputMessage`. It now goes through the `Item` union, with another
  pass over the value.

## Migrating

- A type name that no longer resolves: look it up in the renames below. Where a model has several successors, the one
  you need depends on the field; the compiler names it. For the fields of a struct, use the field's new type.
- A variant name that no longer resolves: most only change case. See the list of variant renames.
- `Null` in a match: remove the arm and handle `None` on the `Option` around the union.
- `ReasoningEffort::TextVariant(ReasoningEffortTextVariantEnum::High)` and its like: `ReasoningEffort::High`. The same
  for `BetaMessagePhase`, `BetaReasoningEffort`, `BetaServiceTierResponses`, `BetaVerbosity`, `MessagePhase`,
  `ServiceTier`, `ServiceTierResponses`, `Verbosity`.
- `response.text` on `create_transcription`: match on `CreateTranscription200Response`. All three variants have a
  `text` field.
- A union that became a struct (`AuditLogActor`, `BetaResponseError`, `BetaWebSearchApproximateLocation`,
  `ResponseError`, `WebSearchApproximateLocation`, `BetaPrompt`, `BetaResponseStreamOptions`,
  `ChatCompletionStreamOptions`, `Prompt`, `ResponseStreamOptions`, `RunCompletionUsage`, `RunStepCompletionUsage`):
  read its fields, and build it with `new` or the builder.
- A `serde_json::Value` out of a `RunStreamEvent`, `RunStepStreamEvent`, `MessageStreamEvent` or
  `CreateModerationRequestInputOneOfInner` variant: use the typed fields.
- Code that matches on an error's text, such as `data did not match any variant`: use the messages above, or better,
  the error type.

<details><summary>Model renames (82)</summary>

- `ApplyPatchOperation1` → `ApplyPatchOperation`
- `BetaReasoningEffortTextVariantEnum` → `BetaReasoningEffort`
- `BetaServiceTierResponsesTextVariantEnum` → `BetaServiceTierResponses`
- `BetaVerbosityTextVariantEnum` → `BetaVerbosity`
- `CreateChatCompletionRequestAllOfTools` → `CreateChatCompletionRequestAllOfToolsInner`
- `GrammarFormat` → `CustomToolPropertiesFormatGrammarFormat`, `CustomToolPropertiesFormatGrammarFormatGrammar`
- `McpToolApprovalFilter1` → `McpToolApprovalFilter`
- `Object0` → `AssistantObjectToolResources`
- `Object01` → `ChatCompletionMessageListDataInnerContentPartsInnerImageUrl`
- `Object010` → `CreateThreadRequestToolResources`
- `Object010FileSearch` → `CreateThreadRequestToolResourcesFileSearch`
- `Object012` → `MessageObjectIncompleteDetails`
- `Object013` → `MessageStreamEventObject0`
- `Object014` → `ModifyAssistantRequestToolResources`
- `Object014CodeInterpreter` → `ModifyAssistantRequestToolResourcesCodeInterpreter`
- `Object014FileSearch` → `ModifyAssistantRequestToolResourcesFileSearch`
- `Object015` → `ModifyThreadRequestToolResources`, `ThreadObjectToolResources`
- `Object015FileSearch` → `ModifyThreadRequestToolResourcesFileSearch`, `ThreadObjectToolResourcesFileSearch`
- `Object016` → `RealtimeTranscriptionSessionCreateResponseGaAudioInputTurnDetection`
- `Object017` → `RealtimeTranslationSessionAudioInputTranscription`, `RealtimeTranslationSessionCreateRequestAudioInputTranscription`, `RealtimeTranslationSessionUpdateRequestAudioInputTranscription`
- `Object018` → `RealtimeTranslationSessionAudioInputNoiseReduction`, `RealtimeTranslationSessionCreateRequestAudioInputNoiseReduction`, `RealtimeTranslationSessionUpdateRequestAudioInputNoiseReduction`
- `Object019` → `ResponseAllOfIncompleteDetails`
- `Object02` → `ChatCompletionMessageListDataInnerContentPartsInnerInputAudio`
- `Object020` → `RunGraderResponseMetadataTokenUsage`
- `Object021` → `RunStepObjectLastError`
- `Object022` → `RunStepStreamEventObject0`
- `Object023` → `RunStreamEventObject0`
- `Object024` → `UserProjects`
- `Object024DataInner` → `UserProjectsDataInner`
- `Object025` → `VectorStoreFileObjectLastError`
- `Object026` → `BetaWebSearchToolFilters`, `WebSearchToolFilters`
- `Object027` → `CreateModerationRequestInputOneOfInnerObject0`
- `Object027Value` → `ChatkitWorkflowStateVariablesValue`
- `Object03` → `ChatCompletionMessageListDataInnerContentPartsInnerFile`
- `Object04` → `ChatCompletionRequestAssistantMessageAudio`
- `Object05` → `ChatCompletionRequestAssistantMessageFunctionCall`
- `Object06` → `ChatCompletionResponseMessageAudio`
- `Object07` → `CreateAssistantRequestToolResources`
- `Object07CodeInterpreter` → `CreateThreadAndRunRequestToolResourcesCodeInterpreter`, `CreateThreadRequestToolResourcesCodeInterpreter`, `ModifyThreadRequestToolResourcesCodeInterpreter`, `ThreadObjectToolResourcesCodeInterpreter`
- `Object07FileSearch` → `CreateAssistantRequestToolResourcesFileSearch`
- `Object07FileSearchVectorStoresInner` → `CreateAssistantRequestToolResourcesFileSearchVectorStoresInner`, `CreateThreadRequestToolResourcesFileSearchVectorStoresInner`
- `Object07FileSearchVectorStoresInnerChunkingStrategy` → `CreateAssistantRequestToolResourcesFileSearchVectorStoresInnerChunkingStrategy`, `CreateThreadRequestToolResourcesFileSearchVectorStoresInnerChunkingStrategy`
- `Object08` → `CreateChatCompletionResponseChoicesInnerLogprobs`
- `Object09` → `CreateCompletionResponseChoicesInnerLogprobs`
- `Object0CodeInterpreter` → `AssistantObjectToolResourcesCodeInterpreter`
- `Object0FileSearch` → `AssistantObjectToolResourcesFileSearch`, `CreateThreadAndRunRequestToolResourcesFileSearch`
- `Object1` → `LiveSessionCreateResponseTransportObject1`
- `Object11` → `MessageStreamEventObject1`
- `Object12` → `RunStepStreamEventObject1`
- `Object13` → `RunStreamEventObject1`
- `Object15` → `CreateModerationRequestInputOneOfInnerObject1`
- `Object2` → `MessageStreamEventObject2`
- `Object21` → `RunStepStreamEventObject2`
- `Object22` → `RunStreamEventObject2`
- `Object3` → `MessageStreamEventObject3`
- `Object31` → `RunStepStreamEventObject3`
- `Object32` → `RunStreamEventObject3`
- `Object4` → `MessageStreamEventObject4`
- `Object41` → `RunStepStreamEventObject4`
- `Object42` → `RunStreamEventObject4`
- `Object5` → `RunStepStreamEventObject5`
- `Object51` → `RunStreamEventObject5`
- `Object6` → `RunStepStreamEventObject6`
- `Object61` → `RunStreamEventObject6`
- `Object7` → `RunStreamEventObject7`
- `Object8` → `RunStreamEventObject8`
- `Object9` → `RunStreamEventObject9`
- `RealtimeBetaResponseMaxOutputTokens` → `RealtimeResponseMaxOutputTokens`
- `ReasoningEffortTextVariantEnum` → `ReasoningEffort`
- `ServiceTierResponsesTextVariantEnum` → `ServiceTierResponses`
- `ServiceTierTextVariantEnum` → `ServiceTier`
- `SessionConfiguration` → `RealtimeCreateClientSecretRequestSession`
- `SessionConfiguration1` → `RealtimeCreateClientSecretResponseSession`
- `ShellCallOutcome1` → `ShellCallOutcome`
- `StaticChunkingStrategyStatic` → `CreateAssistantRequestToolResourcesFileSearchVectorStoresInnerChunkingStrategyStaticChunkingStrategyStatic`, `CreateThreadRequestToolResourcesFileSearchVectorStoresInnerChunkingStrategyStaticChunkingStrategyStatic`
- `TracingConfiguration` → `RealtimeSessionCreateRequestGaTracingTracingConfiguration`, `RealtimeSessionCreateResponseGaTracingTracingConfiguration`, `RealtimeSessionTracing`, `RealtimeSessionTracingTracingConfiguration`
- `TracingConfiguration1` → `RealtimeSessionCreateRequestGaTracing`, `RealtimeSessionCreateResponseGaTracing`
- `VerbosityTextVariantEnum` → `Verbosity`
- `WebhookBatchCancelledData` → `WebhookBatchFailedData`
- `WebhookEvalRunCanceledData` → `WebhookEvalRunFailedData`
- `WebhookFineTuningJobCancelledData` → `WebhookFineTuningJobFailedData`
- `WebhookResponseCancelledData` → `WebhookResponseFailedData`

</details>

<details><summary>Models that no longer stand in other locations (66)</summary>

- `AssistantObjectToolsInner`: other locations now use `CreateAssistantRequestToolsInner`, `CreateRunRequestToolsInner`, `CreateThreadAndRunRequestToolsInner`, `ModifyAssistantRequestToolsInner`, `RunObjectToolsInner`
- `AssistantsNamedToolChoiceFunction`: other locations now use `ChatCompletionNamedToolChoiceFunction`
- `AuditLogCertificateCreated`: other locations now use `AuditLogCertificateUpdated`
- `AuditLogCertificatesActivated`: other locations now use `AuditLogCertificatesDeactivated`
- `AuditLogCertificatesActivatedCertificatesInner`: other locations now use `AuditLogCertificatesDeactivatedCertificatesInner`
- `AuditLogInviteAccepted`: other locations now use `AuditLogInviteDeleted`
- `AuditLogIpAllowlistConfigActivatedConfigsInner`: other locations now use `AuditLogIpAllowlistConfigDeactivatedConfigsInner`
- `AuditLogLoginFailed`: other locations now use `AuditLogLogoutFailed`
- `AuditLogProjectArchived`: other locations now use `AuditLogProjectDeleted`
- `BatchUsageOutputTokensDetails`: other locations now use `ResponseUsageOutputTokensDetails`
- `BetaAgentMessageContentInner`: other locations now use `BetaMessageContentInner`
- `BetaApplyPatchCreateFileOperation`: other locations now use `ApplyPatchCreateFileOperation`
- `BetaApplyPatchDeleteFileOperation`: other locations now use `ApplyPatchDeleteFileOperation`
- `BetaApplyPatchUpdateFileOperation`: other locations now use `ApplyPatchUpdateFileOperation`
- `BetaContainerAutoParamNetworkPolicy`: other locations now use `BetaAutoCodeInterpreterToolParamNetworkPolicy`
- `BetaFunctionShellCallOutputExitOutcome`: other locations now use `FunctionShellCallOutputExitOutcome`
- `BetaFunctionShellCallOutputTimeoutOutcome`: other locations now use `FunctionShellCallOutputTimeoutOutcome`
- `BetaTokenCountsBodyInput`: other locations now use `BetaCompactResponseMethodPublicBodyInput`
- `ChatCompletionNamedToolChoice`: other locations now use `InputMessagesItemReference`, `InputMessagesTemplate`, `ItemReferenceInputMessages`, `PcmAudioFormat`, `PcmaAudioFormat`, `PcmuAudioFormat`, `RetentionRatioTruncation`, `SemanticVad`, `ServerVad`, `TemplateInputMessages`
- `CreateContainerBodyNetworkPolicy`: other locations now use `AutoCodeInterpreterToolParamNetworkPolicy`, `ContainerAutoParamNetworkPolicy`
- `CreateContainerBodySkillsInner`: other locations now use `ContainerAutoParamSkillsInner`
- `CreateFineTuningJobRequestHyperparametersLearningRateMultiplier`: other locations now use `FineTuningJobHyperparametersLearningRateMultiplier`
- `CreateFineTuningJobRequestHyperparametersNEpochs`: other locations now use `FineTuningJobHyperparametersNEpochs`
- `CreateMessageRequestAttachmentsInner`: other locations now use `MessageObjectAttachmentsInner`
- `CreateMessageRequestAttachmentsInnerToolsInner`: other locations now use `MessageObjectAttachmentsInnerToolsInner`
- `DeleteResponse200Response`: other locations now use `BetaDeleteResponse200Response`
- `FineTuneDpoHyperparametersBatchSize`: other locations now use `FineTuneReinforcementHyperparametersBatchSize`, `FineTuneSupervisedHyperparametersBatchSize`
- `FineTuneDpoHyperparametersLearningRateMultiplier`: other locations now use `FineTuneReinforcementHyperparametersLearningRateMultiplier`, `FineTuneSupervisedHyperparametersLearningRateMultiplier`
- `FineTuneDpoHyperparametersNEpochs`: other locations now use `FineTuneReinforcementHyperparametersNEpochs`, `FineTuneSupervisedHyperparametersNEpochs`
- `FineTuneReinforcementMethodGrader`: other locations now use `RunGraderRequestGrader`, `ValidateGraderRequestGrader`, `ValidateGraderResponseGrader`
- `ImageEditCompletedEventSize`: other locations now use `ImageEditPartialImageEventSize`, `ImageGenCompletedEventSize`, `ImageGenPartialImageEventSize`
- `ImageGenToolCallSize`: other locations now use `BetaImageGenToolCallSize`
- `ImageGenToolInputImageMask`: other locations now use `BetaImageGenToolInputImageMask`
- `InviteProjectsInner`: other locations now use `InviteRequestProjectsInner`
- `LiveResponsesDelegationSettingsInputParamToolChoice`: other locations now use `LiveResponsesDelegationSettingsUpdateInputParamToolChoice`
- `LiveResponsesDelegationSettingsInputParamToolsInner`: other locations now use `LiveResponsesDelegationSettingsUpdateInputParamToolsInner`
- `LiveSessionCreateParamsDelegation`: other locations now use `LiveSessionResourceParamDelegation`
- `OrganizationCertificateCertificateDetails`: other locations now use `OrganizationProjectCertificateCertificateDetails`
- `RealtimeBetaResponseCreateParamsMaxOutputTokens`: other locations now use `RealtimeResponseCreateParamsMaxOutputTokens`, `RealtimeSessionCreateRequestGaMaxOutputTokens`, `RealtimeSessionCreateRequestMaxResponseOutputTokens`, `RealtimeSessionCreateResponseGaMaxOutputTokens`, `RealtimeSessionCreateResponseMaxOutputTokens`, `RealtimeSessionMaxResponseOutputTokens`
- `RealtimeBetaResponseCreateParamsToolChoice`: other locations now use `RealtimeResponseCreateParamsToolChoice`, `RealtimeSessionCreateRequestGaToolChoice`, `RealtimeSessionCreateResponseGaToolChoice`
- `RealtimeBetaResponseStatusDetailsError`: other locations now use `RealtimeResponseStatusDetailsError`
- `RealtimeBetaResponseUsageInputTokenDetailsCachedTokensDetails`: other locations now use `RealtimeResponseUsageInputTokenDetailsCachedTokensDetails`
- `RealtimeBetaResponseUsageOutputTokenDetails`: other locations now use `RealtimeResponseUsageOutputTokenDetails`
- `RealtimeBetaServerEventConversationItemInputAudioTranscriptionFailedError`: other locations now use `RealtimeServerEventConversationItemInputAudioTranscriptionFailedError`
- `RealtimeBetaServerEventErrorError`: other locations now use `RealtimeServerEventErrorError`
- `RealtimeBetaServerEventRateLimitsUpdatedRateLimitsInner`: other locations now use `RealtimeServerEventRateLimitsUpdatedRateLimitsInner`
- `RealtimeBetaServerEventResponseContentPartAddedPart`: other locations now use `RealtimeServerEventResponseContentPartAddedPart`
- `RealtimeBetaServerEventResponseContentPartDonePart`: other locations now use `RealtimeServerEventResponseContentPartDonePart`
- `RealtimeResponseCreateParamsToolsInner`: other locations now use `RealtimeSessionCreateRequestGaToolsInner`, `RealtimeSessionCreateResponseGaToolsInner`
- `RealtimeServerEventSessionCreatedSession`: other locations now use `RealtimeServerEventSessionUpdatedSession`
- `RealtimeSessionCreateRequestGaAudioInput`: other locations now use `RealtimeTranscriptionSessionCreateRequestGaAudioInput`
- `RealtimeSessionCreateRequestTurnDetection`: other locations now use `RealtimeTranscriptionSessionCreateResponseTurnDetection`
- `RealtimeSessionCreateResponseAudioInputNoiseReduction`: other locations now use `RealtimeTranscriptionSessionCreateResponseGaAudioInputNoiseReduction`
- `RealtimeSessionInputAudioNoiseReduction`: other locations now use `RealtimeSessionCreateRequestGaAudioInputNoiseReduction`, `RealtimeTranscriptionSessionCreateRequestGaAudioInputNoiseReduction`, `RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReduction`
- `RealtimeTranslationSessionAudio`: other locations now use `RealtimeTranslationSessionCreateRequestAudio`, `RealtimeTranslationSessionUpdateRequestAudio`
- `RealtimeTranslationSessionAudioInput`: other locations now use `RealtimeTranslationSessionCreateRequestAudioInput`, `RealtimeTranslationSessionUpdateRequestAudioInput`
- `RealtimeTranslationSessionAudioOutput`: other locations now use `RealtimeTranslationSessionCreateRequestAudioOutput`, `RealtimeTranslationSessionUpdateRequestAudioOutput`
- `ResponseReasoningSummaryPartAddedEventPart`: other locations now use `BetaResponseReasoningSummaryPartAddedEventPart`
- `ResponseReasoningSummaryPartDoneEventPart`: other locations now use `BetaResponseReasoningSummaryPartDoneEventPart`
- `StaticChunkingStrategy`: other locations now use `CreateAssistantRequestToolResourcesFileSearchVectorStoresInnerChunkingStrategyStaticChunkingStrategy`, `CreateThreadRequestToolResourcesFileSearchVectorStoresInnerChunkingStrategyStaticChunkingStrategy`
- `TokenCountsBodyInput`: other locations now use `CompactResponseMethodPublicBodyInput`
- `TranscriptTextDeltaEventLogprobsInner`: other locations now use `TranscriptTextDoneEventLogprobsInner`
- `WebhookLiveCallIncomingDataSipHeadersInner`: other locations now use `WebhookLiveTransportIncomingDataSipHeadersInner`, `WebhookRealtimeCallIncomingDataSipHeadersInner`
- `WebhookLiveCallIncomingDataSipMediaSecurity`: other locations now use `WebhookLiveTransportIncomingDataSipMediaSecurity`, `WebhookRealtimeCallIncomingDataSipMediaSecurity`
- `WebhookSafetyAlertCreatedData`: other locations now use `WebhookSafetyOrgAlertCreatedData`
- `WebhookSafetyDeactivationIssuedData`: other locations now use `WebhookSafetyWarningIssuedData`

</details>

<details><summary>Variant renames by union (308 in 119 unions)</summary>

- `AgentContentResource`: `EncryptedContentResource` → `EncryptedContent`, `OutputTextResource` → `OutputText`
- `AgentOutputItemResource`: `AssistantMessageItemResource` → `Message`, `BrowserAuthenticationRequestItemResource` → `ComputerUseApprovalRequest`, `CloseSubagentCallItemResource` → `CloseSubagentCall`, `CommandExecutionItemResource` → `CommandExecution`, `ComputerUseCallItemResource` → `ComputerUseCall`, `CreateSubagentCallItemResource` → `CreateSubagentCall`, `FunctionCallItemResource` → `FunctionCall`, `InterruptSubagentCallItemResource` → `InterruptSubagentCall`, `McpCallItemResource` → `McpCall`, `ReasoningItemResource` → `Reasoning`, `ResumeSubagentCallItemResource` → `ResumeSubagentCall`, `SendSubagentInputCallItemResource` → `SendSubagentInputCall`, `WaitForSubagentsCallItemResource` → `WaitForSubagentsCall`, `WebSearchCallItemResource` → `WebSearchCall`
- `ApplyPatchOperation1 → ApplyPatchOperation`: `BetaApplyPatchCreateFileOperation` → `ApplyPatchCreateFileOperation`, `BetaApplyPatchDeleteFileOperation` → `ApplyPatchDeleteFileOperation`, `BetaApplyPatchUpdateFileOperation` → `ApplyPatchUpdateFileOperation`
- `AssistantStreamEvent`: `Doneevent` → `DoneEvent`, `Errorevent` → `ErrorEvent`, `Messagestreamevent` → `MessageStreamEvent`, `Runstepstreamevent` → `RunStepStreamEvent`, `Runstreamevent` → `RunStreamEvent`, `Threadstreamevent` → `ThreadStreamEvent`
- `AssistantsApiResponseFormatOption`: `Responseformatjsonobject` → `ResponseFormatJsonObject`, `Responseformatjsonschema` → `ResponseFormatJsonSchema`, `Responseformattext` → `ResponseFormatText`
- `AssistantsApiToolChoiceOption`: `Assistantsnamedtoolchoice` → `AssistantsNamedToolChoice`
- `BetaCodeInterpreterToolContainer`: `Betaautocodeinterpretertoolparam` → `BetaAutoCodeInterpreterToolParam`
- `BetaComparisonFilterValue`: `Arrayofitems` → `ArrayOfItems`
- `BetaContent`: `Betainputcontent` → `BetaInputContent`, `Betaoutputcontent` → `BetaOutputContent`
- `BetaConversationParam`: `Betaconversationparam2` → `BetaConversationParam2`
- `BetaEasyInputMessageContent`: `Betainputmessagecontentlist` → `BetaInputMessageContentList`
- `BetaFilters`: `Betacomparisonfilter` → `BetaComparisonFilter`, `Betacompoundfilter` → `BetaCompoundFilter`
- `BetaFunctionCallOutputItemParamOutput`: `Arrayofitems` → `ArrayOfItems`
- `BetaFunctionShellCallEnvironment`: `Betacontainerreferenceresource` → `BetaContainerReferenceResource`, `Betalocalenvironmentresource` → `BetaLocalEnvironmentResource`
- `BetaFunctionShellCallItemParamEnvironment`: `Betacontainerreferenceparam` → `BetaContainerReferenceParam`, `Betalocalenvironmentparam` → `BetaLocalEnvironmentParam`
- `BetaFunctionShellToolParamEnvironment`: `Betacontainerautoparam` → `BetaContainerAutoParam`, `Betacontainerreferenceparam` → `BetaContainerReferenceParam`, `Betalocalenvironmentparam` → `BetaLocalEnvironmentParam`
- `BetaMcpToolAllowedTools`: `ArrayOfStrings` → `McpAllowedTools`, `Betamcptoolfilter` → `BetaMcpToolFilter`
- `BetaResponseSteerInput`: `Betaresponsesteerinputitemlist` → `BetaResponseSteerInputItemList`
- `BetaResponseSteerRequiredInput`: `AnyOf` → `RequiredFunctionToolCallOutput`, `AnyOf1` → `RequiredCustomToolCallOutput`, `AnyOf2` → `RequiredComputerToolCallOutput`, `AnyOf3` → `RequiredShellToolCallOutput`, `AnyOf4` → `RequiredApplyPatchToolCallOutput`, `AnyOf5` → `RequiredToolSearchOutput`, `AnyOf6` → `RequiredMcpApprovalResponse`
- `BetaResponsesClientEvent`: `ResponseCreate` → `BetaResponsesClientEventResponseCreate`
- `BetaTextResponseFormatConfiguration`: `Betaresponseformatjsonobject` → `BetaResponseFormatJsonObject`, `Betaresponseformattext` → `BetaResponseFormatText`, `Betatextresponseformatjsonschema` → `BetaTextResponseFormatJsonSchema`
- `BetaTokenCountsBodyInput`: `Arrayofbetainputitems` → `ArrayOfBetaInputItems`
- `BetaTokenCountsBodyInput → BetaCompactResponseMethodPublicBodyInput`: `Arrayofbetainputitems` → `ArrayOfBetaInputItems`
- `BetaTool`: `SearchToolParam` → `BetaToolSearchToolParam`
- `BetaToolChoiceParam`: `Betaspecificapplypatchparam` → `BetaSpecificApplyPatchParam`, `Betaspecificfunctionshellparam` → `BetaSpecificFunctionShellParam`, `Betaspecificprogrammatictoolcallingparam` → `BetaSpecificProgrammaticToolCallingParam`, `Betatoolchoiceallowed` → `BetaToolChoiceAllowed`, `Betatoolchoicecustom` → `BetaToolChoiceCustom`, `Betatoolchoicefunction` → `BetaToolChoiceFunction`, `Betatoolchoicemcp` → `BetaToolChoiceMcp`, `Betatoolchoiceoptions` → `BetaToolChoiceOptions`, `Betatoolchoicetypes` → `BetaToolChoiceTypes`
- `BetaUserMessageItemParamContent`: `Arrayofitems` → `ArrayOfItems`
- `ChatCompletionRequestUserMessageContentPart`: `Chatcompletionrequestmessagecontentpartaudio` → `ChatCompletionRequestMessageContentPartAudio`, `Chatcompletionrequestmessagecontentpartfile` → `ChatCompletionRequestMessageContentPartFile`, `Chatcompletionrequestmessagecontentpartimage` → `ChatCompletionRequestMessageContentPartImage`, `Chatcompletionrequestmessagecontentparttext` → `ChatCompletionRequestMessageContentPartText`
- `ChatCompletionToolChoiceOption`: `Chatcompletionallowedtoolschoice` → `ChatCompletionAllowedToolsChoice`, `Chatcompletionnamedtoolchoice` → `ChatCompletionNamedToolChoice`, `Chatcompletionnamedtoolchoicecustom` → `ChatCompletionNamedToolChoiceCustom`
- `CodeInterpreterToolContainer`: `Autocodeinterpretertoolparam` → `AutoCodeInterpreterToolParam`
- `ComparisonFilterValue`: `Arrayofitems` → `ArrayOfItems`
- `ComputerUseApprovalResponseKindResource`: `BrowserAuthenticationCancelResource` → `Cancel`, `BrowserAuthenticationSubmitResource` → `Submit`
- `ComputerUseApprovalResponseParam`: `BrowserOriginAccessParam` → `BrowserOriginAccess`
- `ComputerUseApprovalResponseParamBrowserAuthentication`: `CancelParam` → `Cancel`, `SubmitParam` → `Submit`
- `Content`: `Inputcontent` → `InputContent`, `Outputcontent` → `OutputContent`
- `ConversationParam`: `Conversationparam2` → `ConversationParam2`
- `CreateEvalCompletionsRunDataSourceInputMessages`: `Itemreferenceinputmessages` → `ItemReferenceInputMessages`, `Templateinputmessages` → `TemplateInputMessages`
- `CreateEvalCompletionsRunDataSourceSource`: `Evaljsonlfilecontentsource` → `EvalJsonlFileContentSource`, `Evaljsonlfileidsource` → `EvalJsonlFileIdSource`, `Evalstoredcompletionssource` → `EvalStoredCompletionsSource`
- `CreateEvalItem`: `Evalitem` → `EvalItem`
- `CreateEvalJsonlRunDataSourceSource`: `Evaljsonlfilecontentsource` → `EvalJsonlFileContentSource`, `Evaljsonlfileidsource` → `EvalJsonlFileIdSource`
- `CreateEvalRequestDataSourceConfig`: `Createevalcustomdatasourceconfig` → `CreateEvalCustomDataSourceConfig`, `Createevallogsdatasourceconfig` → `CreateEvalLogsDataSourceConfig`, `Createevalstoredcompletionsdatasourceconfig` → `CreateEvalStoredCompletionsDataSourceConfig`
- `CreateEvalResponsesRunDataSourceInputMessages`: `Inputmessagesitemreference` → `InputMessagesItemReference`, `Inputmessagestemplate` → `InputMessagesTemplate`
- `CreateEvalResponsesRunDataSourceSource`: `Evaljsonlfilecontentsource` → `EvalJsonlFileContentSource`, `Evaljsonlfileidsource` → `EvalJsonlFileIdSource`, `Evalresponsessource` → `EvalResponsesSource`
- `CreateEvalRunRequestDataSource`: `Createevalcompletionsrundatasource` → `CreateEvalCompletionsRunDataSource`, `Createevaljsonlrundatasource` → `CreateEvalJsonlRunDataSource`, `Createevalresponsesrundatasource` → `CreateEvalResponsesRunDataSource`
- `CreateModerationRequestInput`: `Arrayofitems` → `ArrayOfItems`
- `CreateModerationRequestInputOneOfInner`: `Object027` → `Object0`, `Object15` → `Object1`
- `CreateSessionInputParam`: `Arrayofinputmessageparams` → `ArrayOfInputMessageParams`
- `CreateTranscriptionRequestChunkingStrategy`: `Vadconfig` → `VadConfig`
- `CreateTranscriptionResponseJsonUsage`: `Transcripttextusageduration` → `TranscriptTextUsageDuration`, `Transcripttextusagetokens` → `TranscriptTextUsageTokens`
- `CreateVectorStoreRequestChunkingStrategy`: `Autochunkingstrategyrequestparam` → `AutoChunkingStrategyRequestParam`, `Staticchunkingstrategyrequestparam` → `StaticChunkingStrategyRequestParam`
- `CreateVideoEditMultipartBodyVideo`: `Videoreferenceinputparam` → `VideoReferenceInputParam`
- `CreateVideoExtendMultipartBodyVideo`: `Videoreferenceinputparam` → `VideoReferenceInputParam`
- `CreateVideoMultipartBodyInputReference`: `Imagerefparam2` → `ImageRefParam2`
- `DecisionInput`: `Arrayofitems` → `ArrayOfItems`
- `DecisionInputContent`: `Decisioninputcontentparts` → `DecisionInputContentParts`
- `DecisionInputContentPartsInner`: `DecisionInputImage` → `InputImage`, `DecisionInputOutputText` → `OutputText`, `DecisionInputRefusal` → `Refusal`, `DecisionInputText` → `InputText`
- `DecisionInputToolOutputContent`: `Decisioninputtooloutputcontentparts` → `DecisionInputToolOutputContentParts`
- `DecisionInputToolOutputContentPartsInner`: `DecisionInputImage` → `InputImage`, `DecisionInputText` → `InputText`
- `DecisionsUsagePermissionErrorResponseError`: `Decisionsusageerrordetails` → `DecisionsUsageErrorDetails`
- `Delegation`: `Liveclientdelegationparam` → `LiveClientDelegationParam`, `Livedecisionsdelegationparam` → `LiveDecisionsDelegationParam`, `Liveresponsesdelegationparam` → `LiveResponsesDelegationParam`
- `EasyInputMessageContent`: `Inputmessagecontentlist` → `InputMessageContentList`
- `EmbeddingEmbedding`: `Arrayofnumbers` → `ArrayOfNumbers`
- `EvalDataSourceConfig`: `Evalcustomdatasourceconfig` → `EvalCustomDataSourceConfig`, `Evallogsdatasourceconfig` → `EvalLogsDataSourceConfig`, `Evalstoredcompletionsdatasourceconfig` → `EvalStoredCompletionsDataSourceConfig`
- `EvalItemContent`: `Evalitemcontentarray` → `EvalItemContentArray`, `Evalitemcontentitem` → `EvalItemContentItem`
- `EvalItemContentItem`: `Evalitemcontentoutputtext` → `EvalItemContentOutputText`, `Evaliteminputimage` → `EvalItemInputImage`, `Inputaudio` → `InputAudio`, `Inputtextcontent` → `InputTextContent`
- `EvalRunDataSource`: `Createevalcompletionsrundatasource` → `CreateEvalCompletionsRunDataSource`, `Createevaljsonlrundatasource` → `CreateEvalJsonlRunDataSource`, `Createevalresponsesrundatasource` → `CreateEvalResponsesRunDataSource`
- `Filters`: `Comparisonfilter` → `ComparisonFilter`, `Compoundfilter` → `CompoundFilter`
- `FineTuneReinforcementMethodGrader`: `Gradermulti` → `GraderMulti`, `Graderpython` → `GraderPython`, `Graderscoremodel` → `GraderScoreModel`, `Graderstringcheck` → `GraderStringCheck`, `Gradertextsimilarity` → `GraderTextSimilarity`
- `FineTuneReinforcementMethodGrader → RunGraderRequestGrader`: `Gradermulti` → `GraderMulti`, `Graderpython` → `GraderPython`, `Graderscoremodel` → `GraderScoreModel`, `Graderstringcheck` → `GraderStringCheck`, `Gradertextsimilarity` → `GraderTextSimilarity`
- `FineTuneReinforcementMethodGrader → ValidateGraderRequestGrader`: `Gradermulti` → `GraderMulti`, `Graderpython` → `GraderPython`, `Graderscoremodel` → `GraderScoreModel`, `Graderstringcheck` → `GraderStringCheck`, `Gradertextsimilarity` → `GraderTextSimilarity`
- `FineTuneReinforcementMethodGrader → ValidateGraderResponseGrader`: `Gradermulti` → `GraderMulti`, `Graderpython` → `GraderPython`, `Graderscoremodel` → `GraderScoreModel`, `Graderstringcheck` → `GraderStringCheck`, `Gradertextsimilarity` → `GraderTextSimilarity`
- `FunctionCallOutputItemParamOutput`: `Arrayofitems` → `ArrayOfItems`
- `FunctionCallOutputParam`: `Arrayofinputcontentparams` → `ArrayOfInputContentParams`
- `FunctionCallOutputResource`: `Arrayofinputcontentresources` → `ArrayOfInputContentResources`
- `FunctionShellCallEnvironment`: `Containerreferenceresource` → `ContainerReferenceResource`, `Localenvironmentresource` → `LocalEnvironmentResource`
- `FunctionShellCallItemParamEnvironment`: `Containerreferenceparam` → `ContainerReferenceParam`, `Localenvironmentparam` → `LocalEnvironmentParam`
- `FunctionShellToolParamEnvironment`: `Containerautoparam` → `ContainerAutoParam`, `Containerreferenceparam` → `ContainerReferenceParam`, `Localenvironmentparam` → `LocalEnvironmentParam`
- `LiveDataChannelConfigParamAllowedServerEvents`: `Arrayofliveallowedservereventparams` → `ArrayOfLiveAllowedServerEventParams`
- `LiveDecisionInputItemParamContent`: `Arrayofitems` → `ArrayOfItems`
- `LiveDecisionStoredInputItemParamContent`: `Arrayofitems` → `ArrayOfItems`
- `LiveHostedShellContainerAutoParamNetworkPolicy`: `Livecontainernetworkpolicydisabledparam` → `LiveContainerNetworkPolicyDisabledParam`, `Livehostedshellnetworkpolicyallowlistparam` → `LiveHostedShellNetworkPolicyAllowlistParam`
- `LiveHostedShellToolInputParamEnvironment`: `Livecontainerreferenceparam` → `LiveContainerReferenceParam`, `Livehostedshellcontainerautoparam` → `LiveHostedShellContainerAutoParam`, `Livelocalenvironmentparam` → `LiveLocalEnvironmentParam`
- `LiveResponsesDelegationSettingsInputParamToolChoice`: `Liveallowedtoolschoiceparam` → `LiveAllowedToolsChoiceParam`, `Livetoolchoiceenum` → `LiveToolChoiceEnum`
- `LiveResponsesDelegationSettingsInputParamToolChoice → LiveResponsesDelegationSettingsUpdateInputParamToolChoice`: `Liveallowedtoolschoiceparam` → `LiveAllowedToolsChoiceParam`, `Livetoolchoiceenum` → `LiveToolChoiceEnum`
- `LiveSessionCreateParamsDelegation`: `Liveclientdelegationparam` → `LiveClientDelegationParam`, `Livedecisionsdelegationparam2` → `LiveDecisionsDelegationParam2`, `Liveresponsesdelegationparam` → `LiveResponsesDelegationParam`
- `LiveSessionCreateParamsDelegation → LiveSessionResourceParamDelegation`: `Liveclientdelegationparam` → `LiveClientDelegationParam`, `Livedecisionsdelegationparam2` → `LiveDecisionsDelegationParam2`, `Liveresponsesdelegationparam` → `LiveResponsesDelegationParam`
- `LiveSessionUpdateParamsDelegation`: `Liveclientdelegationparam` → `LiveClientDelegationParam`, `Livedecisionsdelegationupdateparam` → `LiveDecisionsDelegationUpdateParam`, `Liveresponsesdelegationupdateparam` → `LiveResponsesDelegationUpdateParam`
- `McpToolAllowedTools`: `ArrayOfStrings` → `McpAllowedTools`, `Mcptoolfilter` → `McpToolFilter`
- `MessageStreamEvent`: `Object013` → `Object0`, `Object11` → `Object1`
- `RealtimeBetaResponseCreateParamsToolChoice`: `Toolchoicefunction` → `ToolChoiceFunction`, `Toolchoicemcp` → `ToolChoiceMcp`, `Toolchoiceoptions` → `ToolChoiceOptions`
- `RealtimeBetaResponseCreateParamsToolChoice → RealtimeResponseCreateParamsToolChoice`: `Toolchoicefunction` → `ToolChoiceFunction`, `Toolchoicemcp` → `ToolChoiceMcp`, `Toolchoiceoptions` → `ToolChoiceOptions`
- `RealtimeBetaResponseCreateParamsToolChoice → RealtimeSessionCreateRequestGaToolChoice`: `Toolchoicefunction` → `ToolChoiceFunction`, `Toolchoicemcp` → `ToolChoiceMcp`, `Toolchoiceoptions` → `ToolChoiceOptions`
- `RealtimeBetaResponseCreateParamsToolChoice → RealtimeSessionCreateResponseGaToolChoice`: `Toolchoicefunction` → `ToolChoiceFunction`, `Toolchoicemcp` → `ToolChoiceMcp`, `Toolchoiceoptions` → `ToolChoiceOptions`
- `RealtimeBetaServerEventConversationItemInputAudioTranscriptionCompletedUsage`: `Transcripttextusageduration` → `TranscriptTextUsageDuration`, `Transcripttextusagetokens` → `TranscriptTextUsageTokens`
- `RealtimeClientEventSessionUpdateSession`: `Realtimesessioncreaterequestga` → `RealtimeSessionCreateRequestGa`, `Realtimetranscriptionsessioncreaterequestga` → `RealtimeTranscriptionSessionCreateRequestGa`
- `RealtimeConversationItem`: `FunctionCall` → `RealtimeConversationItemFunctionCall`, `FunctionCallOutput` → `RealtimeConversationItemFunctionCallOutput`, `MessageAssistant` → `RealtimeConversationItemMessageAssistant`, `MessageSystem` → `RealtimeConversationItemMessageSystem`, `MessageUser` → `RealtimeConversationItemMessageUser`
- `RealtimeMcpToolCallError`: `Realtimemcphttperror` → `RealtimeMcphttpError`, `Realtimemcpprotocolerror` → `RealtimeMcpProtocolError`, `Realtimemcptoolexecutionerror` → `RealtimeMcpToolExecutionError`
- `RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsage`: `Transcripttextusageduration` → `TranscriptTextUsageDuration`, `Transcripttextusagetokens` → `TranscriptTextUsageTokens`
- `RealtimeServerEventSessionCreatedSession`: `Realtimesessioncreateresponsega` → `RealtimeSessionCreateResponseGa`, `Realtimetranscriptionsessioncreateresponsega` → `RealtimeTranscriptionSessionCreateResponseGa`
- `RealtimeServerEventSessionCreatedSession → RealtimeServerEventSessionUpdatedSession`: `Realtimesessioncreateresponsega` → `RealtimeSessionCreateResponseGa`, `Realtimetranscriptionsessioncreateresponsega` → `RealtimeTranscriptionSessionCreateResponseGa`
- `RealtimeTranslationClientEvent`: `InputAudioBufferAppend` → `RealtimeTranslationClientEventInputAudioBufferAppend`, `SessionClose` → `RealtimeTranslationClientEventSessionClose`, `SessionUpdate` → `RealtimeTranslationClientEventSessionUpdate`
- `RealtimeTranslationServerEvent`: `SessionClosed` → `RealtimeTranslationServerEventSessionClosed`, `SessionCreated` → `RealtimeTranslationServerEventSessionCreated`, `SessionInputTranscriptDelta` → `RealtimeTranslationServerEventSessionInputTranscriptDelta`, `SessionOutputAudioDelta` → `RealtimeTranslationServerEventSessionOutputAudioDelta`, `SessionOutputTranscriptDelta` → `RealtimeTranslationServerEventSessionOutputTranscriptDelta`, `SessionUpdated` → `RealtimeTranslationServerEventSessionUpdated`
- `ResponseSteerInput`: `Responsesteerinputitemlist` → `ResponseSteerInputItemList`
- `ResponseSteerRequiredInput`: `AnyOf` → `RequiredFunctionToolCallOutput`, `AnyOf1` → `RequiredCustomToolCallOutput`, `AnyOf2` → `RequiredComputerToolCallOutput`, `AnyOf3` → `RequiredShellToolCallOutput`, `AnyOf4` → `RequiredApplyPatchToolCallOutput`, `AnyOf5` → `RequiredToolSearchOutput`, `AnyOf6` → `RequiredMcpApprovalResponse`
- `ResponsesClientEvent`: `ResponseCreate` → `ResponsesClientEventResponseCreate`
- `RunStepObjectStepDetails`: `Runstepdetailsmessagecreationobject` → `RunStepDetailsMessageCreationObject`, `Runstepdetailstoolcallsobject` → `RunStepDetailsToolCallsObject`
- `RunStepStreamEvent`: `Object022` → `Object0`, `Object12` → `Object1`, `Object21` → `Object2`, `Object31` → `Object3`, `Object41` → `Object4`
- `RunStreamEvent`: `Object023` → `Object0`, `Object13` → `Object1`, `Object22` → `Object2`, `Object32` → `Object3`, `Object42` → `Object4`, `Object51` → `Object5`, `Object61` → `Object6`
- `SessionTurnItemResource`: `AgentMessageItemResource` → `AgentMessage`, `BrowserAuthenticationRequestItemResource` → `ComputerUseApprovalRequest`, `CloseSubagentCallItemResource` → `CloseSubagentCall`, `CommandExecutionItemResource` → `CommandExecution`, `ComputerUseApprovalRequestResultItemResource` → `ComputerUseApprovalRequestResult`, `ComputerUseCallItemResource` → `ComputerUseCall`, `CreateSubagentCallItemResource` → `CreateSubagentCall`, `FunctionCallItemResource` → `FunctionCall`, `FunctionCallOutputItemResource` → `FunctionCallOutput`, `InterruptSubagentCallItemResource` → `InterruptSubagentCall`, `McpCallItemResource` → `McpCall`, `MessageItemResource` → `Message`, `ReasoningItemResource` → `Reasoning`, `ResumeSubagentCallItemResource` → `ResumeSubagentCall`, `SendSubagentInputCallItemResource` → `SendSubagentInputCall`, `WaitForSubagentsCallItemResource` → `WaitForSubagentsCall`, `WebSearchCallItemResource` → `WebSearchCall`
- `ShellCallOutcome1 → ShellCallOutcome`: `BetaFunctionShellCallOutputExitOutcome` → `FunctionShellCallOutputExitOutcome`, `BetaFunctionShellCallOutputTimeoutOutcome` → `FunctionShellCallOutputTimeoutOutcome`
- `TextResponseFormatConfiguration`: `Responseformatjsonobject` → `ResponseFormatJsonObject`, `Responseformattext` → `ResponseFormatText`, `Textresponseformatjsonschema` → `TextResponseFormatJsonSchema`
- `TokenCountsBodyInput`: `Arrayofinputitems` → `ArrayOfInputItems`
- `TokenCountsBodyInput → CompactResponseMethodPublicBodyInput`: `Arrayofinputitems` → `ArrayOfInputItems`
- `Tool`: `SearchToolParam` → `ToolSearchToolParam`
- `ToolChoiceParam`: `Specificapplypatchparam` → `SpecificApplyPatchParam`, `Specificfunctionshellparam` → `SpecificFunctionShellParam`, `Specificprogrammatictoolcallingparam` → `SpecificProgrammaticToolCallingParam`, `Toolchoiceallowed` → `ToolChoiceAllowed`, `Toolchoicecustom` → `ToolChoiceCustom`, `Toolchoicefunction` → `ToolChoiceFunction`, `Toolchoicemcp` → `ToolChoiceMcp`, `Toolchoiceoptions` → `ToolChoiceOptions`, `Toolchoicetypes` → `ToolChoiceTypes`
- `TranscriptionChunkingStrategy`: `Vadconfig` → `VadConfig`
- `UserMessageItemParamContent`: `Arrayofitems` → `ArrayOfItems`
- `VaultStatusFilterParam`: `Arrayofvaultstatusparams` → `ArrayOfVaultStatusParams`, `Vaultstatusparam` → `VaultStatusParam`
- `VectorStoreFileObjectChunkingStrategy`: `Otherchunkingstrategyresponseparam` → `OtherChunkingStrategyResponseParam`, `Staticchunkingstrategyresponseparam` → `StaticChunkingStrategyResponseParam`
- `VectorStoreSearchRequestFilters`: `Comparisonfilter` → `ComparisonFilter`, `Compoundfilter` → `CompoundFilter`

</details>

<details><summary>Variants removed by union (78)</summary>

- `BetaFunctionShellCallEnvironment`: `Null`
- `BetaFunctionShellCallItemParamEnvironment`: `Null`
- `BetaFunctionShellToolParamEnvironment`: `Null`
- `BetaMcpToolAllowedTools`: `Null`
- `BetaMcpToolCallError`: `Betamcptoolcallerror`, `Null`
- `BetaMcpToolRequireApproval`: `Null`
- `BetaMessagePhase`: `Betamessagephase2`, `Null`
- `BetaReasoningEffort`: `TextVariant`, `Null`
- `BetaResponseAllOfInstructions`: `BetaEasyInputMessage`, `BetaItem`, `BetaCompactionTriggerItemParam`, `BetaItemReferenceParam`, `BetaProgramItemParam`, `BetaProgramOutputItemParam`
- `BetaServiceTierResponses`: `TextVariant`, `Null`
- `BetaTokenCountsBodyInput`: `Null`
- `BetaVerbosity`: `TextVariant`, `Null`
- `ChatCompletionRequestAssistantMessageContent`: `Null`
- `CostsResultQuantityUnit`: `Null`
- `CreateChatCompletionRequestAllOfFunctionCall`: `Text`
- `CreateFineTuningJobRequestHyperparametersBatchSize`: `Text`
- `CreateFineTuningJobRequestHyperparametersLearningRateMultiplier`: `Text`
- `CreateFineTuningJobRequestHyperparametersNEpochs`: `Text`
- `CreateImageEditRequestSize`: `Null`
- `CreateImageRequestSize`: `Null`
- `CreateTranscriptionRequestChunkingStrategy`: `Null`
- `Delegation`: `Null`
- `EditImageBodyJsonParamSize`: `Null`
- `FineTuningJobError`: `Null`
- `FineTuningJobHyperparametersBatchSize`: `Text`
- `FunctionShellCallEnvironment`: `Null`
- `FunctionShellCallItemParamEnvironment`: `Null`
- `FunctionShellToolParamEnvironment`: `Null`
- `ImageGenToolCallSize`: `Null`
- `LiveDelegationItemTargetTextVariantEnum`: `Decisions`
- `LiveHostedShellContainerAutoParamNetworkPolicy`: `Null`
- `LiveHostedShellToolInputParamEnvironment`: `Null`
- `LiveResponsesDelegationSettingsInputParamToolChoice`: `Livefunctiontoolchoiceparam`, `Livemcptoolchoiceparam`, `Livespecificfilesearchparam`, `Livespecificwebsearchparam`, `Livespecificwebsearchpreviewparam`, `Livespecificimagegenparam`, `Livespecificcomputerparam`, `Livespecificcodeinterpreterparam`, `Livespecificprogrammatictoolcallingparam`, `Livespecificfunctionshellparam`, `Livespecificcustomtoolparam`, `Livespecificapplypatchparam`
- `LiveSessionClosedReasonTextVariantEnum`: `ConnectionLost`
- `LiveSessionCreateParamsDelegation`: `Null`
- `LiveSessionUpdateParamsDelegation`: `Null`
- `McpToolAllowedTools`: `Null`
- `McpToolCallError`: `Mcptoolcallerror`, `Null`
- `McpToolRequireApproval`: `Null`
- `MessagePhase`: `Messagephase2`, `Null`
- `ModelIdsCompaction`: `TextString`, `Null`
- `RealtimeMcpToolCallError`: `Null`
- `RealtimeTurnDetection`: `Null`
- `ReasoningEffort`: `TextVariant`, `Null`
- `ServiceTier`: `TextVariant`, `Null`
- `ServiceTierResponses`: `TextVariant`, `Null`
- `StopConfiguration`: `Null`
- `TokenCountsBodyInput`: `Null`
- `TracingConfiguration`: `Text`
- `Verbosity`: `TextVariant`, `Null`

</details>

<details><summary>Variants added by union (259)</summary>

- `BetaComparisonFilterValue`: `Boolean`, `Number`
- `BetaMcpToolCallError`: `BetaHttpError`, `BetaMcpProtocolError`, `BetaMcpToolExecutionError`
- `BetaMcpToolRequireApproval`: `McpToolApprovalFilter`
- `BetaMessagePhase`: `Commentary`, `FinalAnswer`
- `BetaReasoningEffort`: `High`, `Low`, `Max`, `Medium`, `Minimal`, `None`, `Xhigh`
- `BetaResponseAllOfInstructions`: `InputItemList`, `Text`
- `BetaResponsesServerEvent`: `BetaResponseAudioTranscriptWsDelta`, `BetaResponseAudioTranscriptWsDone`, `BetaResponseAudioWsDelta`, `BetaResponseAudioWsDone`, `BetaResponseCodeInterpreterCallCodeWsDelta`, `BetaResponseCodeInterpreterCallCodeWsDone`, `BetaResponseCodeInterpreterCallInWsProgress`, `BetaResponseCodeInterpreterCallWsCompleted`, `BetaResponseCodeInterpreterCallWsInterpreting`, `BetaResponseCompactionWsCompacting`, `BetaResponseContentPartWsAdded`, `BetaResponseContentPartWsDone`, `BetaResponseCustomToolCallInputWsDelta`, `BetaResponseCustomToolCallInputWsDone`, `BetaResponseFileSearchCallInWsProgress`, `BetaResponseFileSearchCallWsCompleted`, `BetaResponseFileSearchCallWsSearching`, `BetaResponseFunctionCallArgumentsWsDelta`, `BetaResponseFunctionCallArgumentsWsDone`, `BetaResponseImageGenCallInWsProgress`, `BetaResponseImageGenCallPartialWsImage`, `BetaResponseImageGenCallWsCompleted`, `BetaResponseImageGenCallWsGenerating`, `BetaResponseInWsProgress`, `BetaResponseInjectCreatedEvent`, `BetaResponseInjectFailedEvent`, `BetaResponseMcpCallArgumentsWsDelta`, `BetaResponseMcpCallArgumentsWsDone`, `BetaResponseMcpCallInWsProgress`, `BetaResponseMcpCallWsCompleted`, `BetaResponseMcpCallWsFailed`, `BetaResponseMcpListToolsInWsProgress`, `BetaResponseMcpListToolsWsCompleted`, `BetaResponseMcpListToolsWsFailed`, `BetaResponseOutputItemWsAdded`, `BetaResponseOutputItemWsDone`, `BetaResponseOutputTextAnnotationWsAdded`, `BetaResponseReasoningSummaryPartWsAdded`, `BetaResponseReasoningSummaryPartWsDone`, `BetaResponseReasoningSummaryTextWsDelta`, `BetaResponseReasoningSummaryTextWsDone`, `BetaResponseReasoningTextWsDelta`, `BetaResponseReasoningTextWsDone`, `BetaResponseRefusalWsDelta`, `BetaResponseRefusalWsDone`, `BetaResponseShellCallCommandWsAdded`, `BetaResponseShellCallCommandWsDelta`, `BetaResponseShellCallCommandWsDone`, `BetaResponseShellCallOutputContentWsDelta`, `BetaResponseShellCallOutputContentWsDone`, `BetaResponseSteerAcceptedEvent`, `BetaResponseSteerFailedEvent`, `BetaResponseSteerPendingEvent`, `BetaResponseTextWsDelta`, `BetaResponseTextWsDone`, `BetaResponseWebSearchCallInWsProgress`, `BetaResponseWebSearchCallWsCompleted`, `BetaResponseWebSearchCallWsSearching`, `BetaResponseWsCompleted`, `BetaResponseWsCreated`, `BetaResponseWsError`, `BetaResponseWsFailed`, `BetaResponseWsIncomplete`, `BetaResponseWsQueued`
- `BetaServiceTierResponses`: `Auto`, `Default`, `Fast`, `Flex`, `Priority`, `Scale`, `Ultrafast`
- `BetaVerbosity`: `High`, `Low`, `Medium`
- `ChoiceValueParam`: `Boolean`
- `ChoiceValueResource`: `Boolean`
- `ComparisonFilterValue`: `Boolean`, `Number`
- `CreateChatCompletionRequestAllOfFunctionCall`: `TextVariant`
- `CreateEvalItem`: `SimpleInputMessage`
- `CreateFineTuningJobRequestHyperparametersBatchSize`: `TextVariant`
- `CreateFineTuningJobRequestHyperparametersLearningRateMultiplier`: `TextVariant`
- `CreateFineTuningJobRequestHyperparametersNEpochs`: `TextVariant`
- `FineTuneDpoHyperparametersBatchSize`: `Integer`
- `FineTuneDpoHyperparametersBeta`: `Number`
- `FineTuneDpoHyperparametersLearningRateMultiplier`: `Number`
- `FineTuneDpoHyperparametersNEpochs`: `Integer`
- `FineTuneReinforcementHyperparametersBatchSize`: `Integer`
- `FineTuneReinforcementHyperparametersComputeMultiplier`: `Number`
- `FineTuneReinforcementHyperparametersEvalInterval`: `Integer`
- `FineTuneReinforcementHyperparametersEvalSamples`: `Integer`
- `FineTuneReinforcementHyperparametersLearningRateMultiplier`: `Number`
- `FineTuneReinforcementHyperparametersNEpochs`: `Integer`
- `FineTuneSupervisedHyperparametersBatchSize`: `Integer`
- `FineTuneSupervisedHyperparametersLearningRateMultiplier`: `Number`
- `FineTuneSupervisedHyperparametersNEpochs`: `Integer`
- `FineTuningJobError`: `Object0`, `Object2`
- `FineTuningJobHyperparametersBatchSize`: `TextVariant`
- `FineTuningJobHyperparametersLearningRateMultiplier`: `TextVariant`
- `FineTuningJobHyperparametersNEpochs`: `TextVariant`
- `LiveDecisionChoiceParamValue`: `Boolean`
- `LiveDelegationItemTarget`: `TextVariant2`, `TextVariant3`
- `LiveDelegationItemTargetTextVariantEnum`: `Client`
- `LiveResponsesDelegationSettingsInputParamToolChoice`: `OneOf`
- `LiveResponsesDelegationSettingsUpdateInputParamToolChoice`: `OneOf`
- `LiveServerEvent`: `LiveCommentaryAppended`, `LiveDecisionEvent`, `LiveDecisionItemCreated`, `LiveDecisionItemDeleted`, `LiveDelegationCreated`, `LiveErrorEvent`, `LiveInfoEvent`, `LiveInputAudioAppend`, `LiveInputAudioMuted`, `LiveInputAudioUnmuted`, `LiveInputTranscriptDelta`, `LiveInstructionsAppended`, `LiveOutputAudioDelta`, `LiveOutputAudioFixturePlayed`, `LiveOutputAudioFixturePlaying`, `LiveOutputTranscriptDelta`, `LiveResponseEvent`, `LiveSessionClosed`, `LiveSessionStarted`, `LiveSessionUpdated`, `LiveSessionUsageUpdated`, `LiveThinkingAppended`, `LiveTransportAnswered`, `LiveTransportDtmfReceived`, `LiveTransportDtmfSend`, `LiveTransportFailed`, `LiveTransportRinging`
- `LiveSessionClosedReason`: `TextVariant2`, `TextVariant3`, `TextVariant4`, `TextVariant5`
- `LiveSessionClosedReasonTextVariantEnum`: `CloseRequested`
- `McpToolCallError`: `HttpError`, `McpProtocolError`, `McpToolExecutionError`
- `McpToolRequireApproval`: `McpToolApprovalFilter`
- `MessagePhase`: `Commentary`, `FinalAnswer`
- `RealtimeBetaResponseCreateParamsMaxOutputTokens`: `Integer`
- `RealtimeResponseCreateParamsMaxOutputTokens`: `Integer`
- `RealtimeResponseMaxOutputTokens`: `Integer`
- `RealtimeSessionCreateRequestGaMaxOutputTokens`: `Integer`
- `RealtimeSessionCreateRequestMaxResponseOutputTokens`: `Integer`
- `RealtimeSessionCreateRequestTracing`: `TracingConfiguration`
- `RealtimeSessionCreateResponseGaMaxOutputTokens`: `Integer`
- `RealtimeSessionCreateResponseMaxOutputTokens`: `Integer`
- `RealtimeSessionCreateResponseTracing`: `TracingConfiguration`
- `RealtimeSessionMaxResponseOutputTokens`: `Integer`
- `RealtimeSessionTracing`: `TextVariant`
- `ReasoningEffort`: `High`, `Low`, `Max`, `Medium`, `Minimal`, `None`, `Xhigh`
- `ResponsesServerEvent`: `ResponseAudioTranscriptWsDelta`, `ResponseAudioTranscriptWsDone`, `ResponseAudioWsDelta`, `ResponseAudioWsDone`, `ResponseCodeInterpreterCallCodeWsDelta`, `ResponseCodeInterpreterCallCodeWsDone`, `ResponseCodeInterpreterCallInWsProgress`, `ResponseCodeInterpreterCallWsCompleted`, `ResponseCodeInterpreterCallWsInterpreting`, `ResponseCompactionWsCompacting`, `ResponseContentPartWsAdded`, `ResponseContentPartWsDone`, `ResponseCustomToolCallInputWsDelta`, `ResponseCustomToolCallInputWsDone`, `ResponseFileSearchCallInWsProgress`, `ResponseFileSearchCallWsCompleted`, `ResponseFileSearchCallWsSearching`, `ResponseFunctionCallArgumentsWsDelta`, `ResponseFunctionCallArgumentsWsDone`, `ResponseImageGenCallInWsProgress`, `ResponseImageGenCallPartialWsImage`, `ResponseImageGenCallWsCompleted`, `ResponseImageGenCallWsGenerating`, `ResponseInWsProgress`, `ResponseMcpCallArgumentsWsDelta`, `ResponseMcpCallArgumentsWsDone`, `ResponseMcpCallInWsProgress`, `ResponseMcpCallWsCompleted`, `ResponseMcpCallWsFailed`, `ResponseMcpListToolsInWsProgress`, `ResponseMcpListToolsWsCompleted`, `ResponseMcpListToolsWsFailed`, `ResponseOutputItemWsAdded`, `ResponseOutputItemWsDone`, `ResponseOutputTextAnnotationWsAdded`, `ResponseReasoningSummaryPartWsAdded`, `ResponseReasoningSummaryPartWsDone`, `ResponseReasoningSummaryTextWsDelta`, `ResponseReasoningSummaryTextWsDone`, `ResponseReasoningTextWsDelta`, `ResponseReasoningTextWsDone`, `ResponseRefusalWsDelta`, `ResponseRefusalWsDone`, `ResponseShellCallCommandWsAdded`, `ResponseShellCallCommandWsDelta`, `ResponseShellCallCommandWsDone`, `ResponseShellCallOutputContentWsDelta`, `ResponseShellCallOutputContentWsDone`, `ResponseSteerAcceptedEvent`, `ResponseSteerFailedEvent`, `ResponseSteerPendingEvent`, `ResponseTextWsDelta`, `ResponseTextWsDone`, `ResponseWebSearchCallInWsProgress`, `ResponseWebSearchCallWsCompleted`, `ResponseWebSearchCallWsSearching`, `ResponseWsCompleted`, `ResponseWsCreated`, `ResponseWsError`, `ResponseWsFailed`, `ResponseWsIncomplete`, `ResponseWsQueued`
- `ServiceTier`: `Auto`, `Default`, `Fast`, `Flex`, `Priority`, `Scale`
- `ServiceTierResponses`: `Auto`, `Default`, `Fast`, `Flex`, `Priority`, `Scale`, `Ultrafast`
- `Verbosity`: `High`, `Low`, `Medium`
- `VoiceIdsOrCustomVoice`: `Object1`

</details>

<details><summary>Types that changed kind (27)</summary>

- `AuditLogActor`: union → struct
- `BetaMessagePhase`: union → enum
- `BetaPrompt`: enum → struct
- `BetaReasoningEffort`: union → enum
- `BetaResponseError`: union → struct
- `BetaResponseStreamOptions`: enum → struct
- `BetaResponsesServerEvent`: enum → union
- `BetaServiceTierResponses`: union → enum
- `BetaVerbosity`: union → enum
- `BetaWebSearchApproximateLocation`: union → struct
- `ChatCompletionStreamOptions`: enum → struct
- `CreateTranscription200Response`: struct → union
- `FineTuningJobError`: enum → union
- `LiveServerEvent`: enum → union
- `MessagePhase`: union → enum
- `Prompt`: enum → struct
- `ReasoningEffort`: union → enum
- `ResponseError`: union → struct
- `ResponseStreamOptions`: enum → struct
- `ResponsesServerEvent`: enum → union
- `RunCompletionUsage`: enum → struct
- `RunStepCompletionUsage`: enum → struct
- `ServiceTier`: union → enum
- `ServiceTierResponses`: union → enum
- `Verbosity`: union → enum
- `WebSearchApproximateLocation`: union → struct
- `WebhookLiveCallIncomingDataSipMediaSecurity`: struct → union

</details>

<details><summary>Variants that held a `serde_json::Value` and now hold their type (30)</summary>

- `RunStreamEvent::Object023` → `RunStreamEvent::Object0(RunStreamEventObject0)`
- `RunStreamEvent::Object13` → `RunStreamEvent::Object1(RunStreamEventObject1)`
- `RunStreamEvent::Object22` → `RunStreamEvent::Object2(RunStreamEventObject2)`
- `RunStreamEvent::Object32` → `RunStreamEvent::Object3(RunStreamEventObject3)`
- `RunStreamEvent::Object42` → `RunStreamEvent::Object4(RunStreamEventObject4)`
- `RunStreamEvent::Object51` → `RunStreamEvent::Object5(RunStreamEventObject5)`
- `RunStreamEvent::Object61` → `RunStreamEvent::Object6(RunStreamEventObject6)`
- `RunStepStreamEvent::Object022` → `RunStepStreamEvent::Object0(RunStepStreamEventObject0)`
- `RunStepStreamEvent::Object12` → `RunStepStreamEvent::Object1(RunStepStreamEventObject1)`
- `RunStepStreamEvent::Object21` → `RunStepStreamEvent::Object2(RunStepStreamEventObject2)`
- `RunStepStreamEvent::Object31` → `RunStepStreamEvent::Object3(RunStepStreamEventObject3)`
- `RunStepStreamEvent::Object41` → `RunStepStreamEvent::Object4(RunStepStreamEventObject4)`
- `MessageStreamEvent::Object013` → `MessageStreamEvent::Object0(MessageStreamEventObject0)`
- `MessageStreamEvent::Object11` → `MessageStreamEvent::Object1(MessageStreamEventObject1)`
- `BetaResponseSteerRequiredInput::AnyOf` → `BetaResponseSteerRequiredInput::RequiredFunctionToolCallOutput(RequiredFunctionToolCallOutput)`
- `BetaResponseSteerRequiredInput::AnyOf1` → `BetaResponseSteerRequiredInput::RequiredCustomToolCallOutput(RequiredCustomToolCallOutput)`
- `BetaResponseSteerRequiredInput::AnyOf2` → `BetaResponseSteerRequiredInput::RequiredComputerToolCallOutput(RequiredComputerToolCallOutput)`
- `BetaResponseSteerRequiredInput::AnyOf3` → `BetaResponseSteerRequiredInput::RequiredShellToolCallOutput(RequiredShellToolCallOutput)`
- `BetaResponseSteerRequiredInput::AnyOf4` → `BetaResponseSteerRequiredInput::RequiredApplyPatchToolCallOutput(RequiredApplyPatchToolCallOutput)`
- `BetaResponseSteerRequiredInput::AnyOf5` → `BetaResponseSteerRequiredInput::RequiredToolSearchOutput(RequiredToolSearchOutput)`
- `BetaResponseSteerRequiredInput::AnyOf6` → `BetaResponseSteerRequiredInput::RequiredMcpApprovalResponse(RequiredMcpApprovalResponse)`
- `CreateModerationRequestInputOneOfInner::Object027` → `CreateModerationRequestInputOneOfInner::Object0(CreateModerationRequestInputOneOfInnerObject0)`
- `CreateModerationRequestInputOneOfInner::Object15` → `CreateModerationRequestInputOneOfInner::Object1(CreateModerationRequestInputOneOfInnerObject1)`
- `ResponseSteerRequiredInput::AnyOf` → `ResponseSteerRequiredInput::RequiredFunctionToolCallOutput(RequiredFunctionToolCallOutput)`
- `ResponseSteerRequiredInput::AnyOf1` → `ResponseSteerRequiredInput::RequiredCustomToolCallOutput(RequiredCustomToolCallOutput)`
- `ResponseSteerRequiredInput::AnyOf2` → `ResponseSteerRequiredInput::RequiredComputerToolCallOutput(RequiredComputerToolCallOutput)`
- `ResponseSteerRequiredInput::AnyOf3` → `ResponseSteerRequiredInput::RequiredShellToolCallOutput(RequiredShellToolCallOutput)`
- `ResponseSteerRequiredInput::AnyOf4` → `ResponseSteerRequiredInput::RequiredApplyPatchToolCallOutput(RequiredApplyPatchToolCallOutput)`
- `ResponseSteerRequiredInput::AnyOf5` → `ResponseSteerRequiredInput::RequiredToolSearchOutput(RequiredToolSearchOutput)`
- `ResponseSteerRequiredInput::AnyOf6` → `ResponseSteerRequiredInput::RequiredMcpApprovalResponse(RequiredMcpApprovalResponse)`

</details>

<details><summary>Unused string enums removed (31)</summary>

- `BetaImageGenToolCallQualityTextVariantEnum`
- `BetaImageGenToolSizeTextVariantEnum`
- `BetaItemReferenceParamTypeTextVariantEnum`
- `BetaLocalShellToolCallOutputStatusTextVariantEnum`
- `BetaModelResponsePropertiesPromptCacheRetentionTextVariantEnum`
- `BetaReasoningContextTextVariantEnum`
- `BetaReasoningGenerateSummaryTextVariantEnum`
- `BetaReasoningSummaryTextVariantEnum`
- `CostsResultApiSourceTextVariantEnum`
- `CreateImageEditRequestBackgroundTextVariantEnum`
- `CreateImageEditRequestOutputFormatTextVariantEnum`
- `CreateImageEditRequestQualityTextVariantEnum`
- `CreateImageRequestBackgroundTextVariantEnum`
- `CreateImageRequestModerationTextVariantEnum`
- `CreateImageRequestOutputFormatTextVariantEnum`
- `CreateImageRequestQualityTextVariantEnum`
- `EditImageBodyJsonParamBackgroundTextVariantEnum`
- `EditImageBodyJsonParamInputFidelityTextVariantEnum`
- `EditImageBodyJsonParamModerationTextVariantEnum`
- `EditImageBodyJsonParamOutputFormatTextVariantEnum`
- `EditImageBodyJsonParamQualityTextVariantEnum`
- `ImageGenToolCallQualityTextVariantEnum`
- `ItemReferenceParamTypeTextVariantEnum`
- `LiveDecisionEventSourceEventTypeTextVariantEnum`
- `LocalShellToolCallOutputStatusTextVariantEnum`
- `ModelResponsePropertiesPromptCacheRetentionTextVariantEnum`
- `ReasoningContextTextVariantEnum`
- `ReasoningGenerateSummaryTextVariantEnum`
- `ReasoningSummaryTextVariantEnum`
- `UsageCompletionsResultApiSourceTextVariantEnum`
- `UsageWebSearchCallsResultApiSourceTextVariantEnum`

</details>

<details><summary>Models removed without a successor (9)</summary>

- `BetaMisalignmentErrorTypeTextVariantEnum`
- `BetaPersonalityEnumTextVariantEnum`
- `BetaReasoningModeEnumTextVariantEnum`
- `BetaResponseSteerErrorCodeTextVariantEnum`
- `BetaResponseSteerPendingReasonTextVariantEnum`
- `Object011`
- `Object027ImageUrl`
- `Object14`
- `RealtimeBetaResponseMaxOutputTokensTextVariantEnum`

</details>

<details><summary>New models with no predecessor (115)</summary>

- `AuditLogCertificateUpdated`
- `AuditLogCertificatesDeactivated`
- `AuditLogCertificatesDeactivatedCertificatesInner`
- `AuditLogInviteDeleted`
- `AuditLogIpAllowlistConfigDeactivatedConfigsInner`
- `AuditLogLogoutFailed`
- `AuditLogProjectDeleted`
- `AutoCodeInterpreterToolParamNetworkPolicy`
- `BetaAutoCodeInterpreterToolParamNetworkPolicy`
- `BetaCompactResponseMethodPublicBodyInput`
- `BetaComparisonFilterValueOneOfInner`
- `BetaDeleteResponse200Response`
- `BetaImageGenToolCallSize`
- `BetaImageGenToolInputImageMask`
- `BetaMessageContentInner`
- `BetaResponseReasoningSummaryPartAddedEventPart`
- `BetaResponseReasoningSummaryPartDoneEventPart`
- `BetaUserMessageItemParamContentOneOfInner`
- `ChatCompletionNamedToolChoiceFunction`
- `CompactResponseMethodPublicBodyInput`
- `ContainerAutoParamNetworkPolicy`
- `ContainerAutoParamSkillsInner`
- `CreateAssistantRequestToolResourcesFileSearchVectorStoresInnerChunkingStrategyStaticChunkingStrategy`
- `CreateAssistantRequestToolsInner`
- `CreateChatCompletionRequestAllOfFunctionCallTextVariantEnum`
- `CreateFineTuningJobRequestHyperparametersBatchSizeTextVariantEnum`
- `CreateFineTuningJobRequestHyperparametersLearningRateMultiplierTextVariantEnum`
- `CreateFineTuningJobRequestHyperparametersNEpochsTextVariantEnum`
- `CreateModerationRequestInputOneOfInnerObject0ImageUrl`
- `CreateRunRequestToolsInner`
- `CreateThreadAndRunRequestToolsInner`
- `CreateThreadRequestToolResourcesFileSearchVectorStoresInnerChunkingStrategyStaticChunkingStrategy`
- `FineTuneReinforcementHyperparametersBatchSize`
- `FineTuneReinforcementHyperparametersLearningRateMultiplier`
- `FineTuneReinforcementHyperparametersNEpochs`
- `FineTuneSupervisedHyperparametersBatchSize`
- `FineTuneSupervisedHyperparametersLearningRateMultiplier`
- `FineTuneSupervisedHyperparametersNEpochs`
- `FineTuningJobErrorObject0`
- `FineTuningJobHyperparametersBatchSizeTextVariantEnum`
- `FineTuningJobHyperparametersLearningRateMultiplier`
- `FineTuningJobHyperparametersLearningRateMultiplierTextVariantEnum`
- `FineTuningJobHyperparametersNEpochs`
- `FineTuningJobHyperparametersNEpochsTextVariantEnum`
- `ImageEditPartialImageEventSize`
- `ImageGenCompletedEventSize`
- `ImageGenPartialImageEventSize`
- `InviteRequestProjectsInner`
- `LiveDecisionStoredInputItemParamContentOneOfInner`
- `LiveDelegationItemTargetTextVariant2Enum`
- `LiveDelegationItemTargetTextVariant3Enum`
- `LiveResponsesDelegationSettingsInputParamToolChoiceOneOf`
- `LiveResponsesDelegationSettingsUpdateInputParamToolChoice`
- `LiveResponsesDelegationSettingsUpdateInputParamToolChoiceOneOf`
- `LiveResponsesDelegationSettingsUpdateInputParamToolsInner`
- `LiveSessionClosedReasonTextVariant2Enum`
- `LiveSessionClosedReasonTextVariant3Enum`
- `LiveSessionClosedReasonTextVariant4Enum`
- `LiveSessionClosedReasonTextVariant5Enum`
- `LiveSessionResourceParamDelegation`
- `MessageObjectAttachmentsInner`
- `MessageObjectAttachmentsInnerToolsInner`
- `ModifyAssistantRequestToolsInner`
- `OrganizationProjectCertificateCertificateDetails`
- `RealtimeResponseCreateParamsMaxOutputTokens`
- `RealtimeResponseCreateParamsToolChoice`
- `RealtimeResponseStatusDetailsError`
- `RealtimeResponseUsageInputTokenDetailsCachedTokensDetails`
- `RealtimeResponseUsageOutputTokenDetails`
- `RealtimeServerEventConversationItemInputAudioTranscriptionFailedError`
- `RealtimeServerEventErrorError`
- `RealtimeServerEventRateLimitsUpdatedRateLimitsInner`
- `RealtimeServerEventResponseContentPartAddedPart`
- `RealtimeServerEventResponseContentPartDonePart`
- `RealtimeServerEventSessionUpdatedSession`
- `RealtimeSessionCreateRequestGaAudioInputNoiseReduction`
- `RealtimeSessionCreateRequestGaMaxOutputTokens`
- `RealtimeSessionCreateRequestGaToolChoice`
- `RealtimeSessionCreateRequestGaToolsInner`
- `RealtimeSessionCreateRequestMaxResponseOutputTokens`
- `RealtimeSessionCreateRequestTracingTracingConfiguration`
- `RealtimeSessionCreateResponseGaMaxOutputTokens`
- `RealtimeSessionCreateResponseGaToolChoice`
- `RealtimeSessionCreateResponseGaToolsInner`
- `RealtimeSessionCreateResponseMaxOutputTokens`
- `RealtimeSessionCreateResponseTracingTracingConfiguration`
- `RealtimeSessionMaxResponseOutputTokens`
- `RealtimeTranscriptionSessionCreateRequestGaAudioInput`
- `RealtimeTranscriptionSessionCreateRequestGaAudioInputNoiseReduction`
- `RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReduction`
- `RealtimeTranscriptionSessionCreateResponseGaAudioInputNoiseReduction`
- `RealtimeTranscriptionSessionCreateResponseTurnDetection`
- `RealtimeTranslationSessionCreateRequestAudio`
- `RealtimeTranslationSessionCreateRequestAudioInput`
- `RealtimeTranslationSessionCreateRequestAudioOutput`
- `RealtimeTranslationSessionUpdateRequestAudio`
- `RealtimeTranslationSessionUpdateRequestAudioInput`
- `RealtimeTranslationSessionUpdateRequestAudioOutput`
- `ResponseUsageOutputTokensDetails`
- `RunGraderRequestGrader`
- `RunObjectToolsInner`
- `TranscriptTextDoneEventLogprobsInner`
- `UserMessageItemParamContentOneOfInner`
- `ValidateGraderRequestGrader`
- `ValidateGraderResponseGrader`
- `VoiceIdsOrCustomVoiceObject1`
- `WebhookLiveCallIncomingDataSipMediaSecurityTextVariantEnum`
- `WebhookLiveTransportIncomingDataSipHeadersInner`
- `WebhookLiveTransportIncomingDataSipMediaSecurity`
- `WebhookLiveTransportIncomingDataSipMediaSecurityTextVariantEnum`
- `WebhookRealtimeCallIncomingDataSipHeadersInner`
- `WebhookRealtimeCallIncomingDataSipMediaSecurity`
- `WebhookRealtimeCallIncomingDataSipMediaSecurityTextVariantEnum`
- `WebhookSafetyOrgAlertCreatedData`
- `WebhookSafetyWarningIssuedData`

</details>
