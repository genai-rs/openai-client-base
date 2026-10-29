# openai-client-base 0.16.1 → 0.17.0: openapi-generator 7.15.0 → 7.25.0

This release moves code generation to openapi-generator 7.25.0 (through the
`timvw-rust` generator image `0.3.0-og7.25.0`). The generated crate changes
where 7.25.0 reads the OpenAPI 3.1 spec more faithfully than 7.15.0 did; no
change below comes from the OpenAI spec itself. The machine-readable report is
`api-diff-0.16.1-to-0.17.0.json` in this folder.

## What stays the same

- Type names: the generator option `titleNaming=false` keeps names derived from
  schema keys and property paths. Without it, 7.25.0 merges titled schemas of
  the same shape and names inline array items after the array's title, which
  would have removed 123 public types.
- Dates and times stay `String` (`useChrono=false`).
- Every API function keeps its name and module; two change a parameter type
  (listed below).

## What changes

Counts: {"model_exports_added": 8, "model_exports_removed": 22, "public_functions_added_or_signature_changed": 35, "public_functions_removed_or_signature_changed": 39, "public_model_field_changes": 289, "public_enum_types_with_variant_changes": 45, "public_enum_variants_added": 30, "public_enum_variants_removed": 45, "public_derive_changes": 52, "serde_field_attribute_changes": 137}

### Types removed (22)

`AgentToolConfigParamMcpConnectionOrigin`, `CreateAssistantRequestDescription`, `CreateAssistantRequestInstructions`, `CreateAssistantRequestName`, `CreateAssistantRequestResponseFormat`, `CreateAssistantRequestTemperature`, `CreateAssistantRequestToolResources`, `CreateAssistantRequestTopP`, `CreateThreadAndRunRequestToolResourcesCodeInterpreter`, `CreateThreadRequestToolResources`, `DecisionInputImageDetail`, `EvalRunOutputItemResultSample`, `LiveDecisionEventSourceEventType`, `Object026DataInner`, `Object028`, `Object029`, `Object029ImageUrl`, `Object029Value`, `PersistedAgentToolConfigParamMcpConnectionOrigin`, `PromptCacheBreakpoint1`, `RealtimeTranslationSessionCreateRequestAudio`, `RealtimeTranslationSessionCreateRequestAudioInput`

### Types added (8)

`CreateEnvironmentParam`, `HostedPluginParam`, `HostedPluginResource`, `InlineCapabilitySourceParam`, `Object024DataInner`, `Object027ImageUrl`, `Object027Value`, `RealtimeSessionCreateResponseTurnDetection`

### Field type changes by cause

- **boxing only (`Box<T>` added or removed; constructors take `T` as before)**: 127
- **nullable now honoured: one more `Option` (`type: [T, null]` / `anyOf [T, null]` in the spec)**: 46
- **other type change**: 41
- **fields of removed/added types**: 75

<details><summary>boxing only (`Box<T>` added or removed; constructors take `T` as before) (127)</summary>

- `AgentResource.multi_agent`: `models::MultiAgentConfigResource` → `Box<models::MultiAgentConfigResource>`
- `AgentResource.reasoning`: `models::ReasoningResource` → `Box<models::ReasoningResource>`
- `AgentResource.text`: `models::TextResource` → `Box<models::TextResource>`
- `AgentSessionActionRequiredPayloadResource.required_action`: `models::AgentSessionRequiredActionPayloadResource` → `Box<models::AgentSessionRequiredActionPayloadResource>`
- `AgentSessionCreatedPayloadResource.connect`: `Option<models::AgentSessionConnectPayloadResource>` → `Option<Box<models::AgentSessionConnectPayloadResource>>`
- `AgentToolConfigParamWebSearch.location`: `Option<Option<models::WebSearchLocationParam>>` → `Option<Option<Box<models::WebSearchLocationParam>>>`
- `AgentToolResourceWebSearch.location`: `Option<models::WebSearchLocationResource>` → `Option<Box<models::WebSearchLocationResource>>`
- `BetaCreateResponse.access_programs`: `Option<models::BetaAccessProgramsParam>` → `Option<Box<models::BetaAccessProgramsParam>>`
- `BetaImageGenTool.input_image_mask`: `Option<models::ImageGenToolInputImageMask>` → `Option<Box<models::ImageGenToolInputImageMask>>`
- `BetaResponsesClientEventResponseCreate.access_programs`: `Option<models::BetaAccessProgramsParam>` → `Option<Box<models::BetaAccessProgramsParam>>`
- `BrowserAuthenticationRequestItemResource.request`: `models::BrowserAuthenticationHistoryRequestKindResourceBrowserAuthentication` → `Box<models::BrowserAuthenticationHistoryRequestKindResourceBrowserAuthentication>`
- `ComputerUseCallItemResource.output`: `Option<models::ComputerScreenshotResource>` → `Option<Box<models::ComputerScreenshotResource>>`
- `CreateAgentParams.multi_agent`: `Option<Option<models::MultiAgentConfigCurrentParam>>` → `Option<Option<Box<models::MultiAgentConfigCurrentParam>>>`
- `CreateAgentParams.reasoning`: `Option<Option<models::ReasoningParam>>` → `Option<Option<Box<models::ReasoningParam>>>`
- `CreateAgentParams.text`: `Option<Option<models::TextParam>>` → `Option<Option<Box<models::TextParam>>>`
- `CreateAgentSessionParams.agent`: `Option<models::SessionAgentConfigParam>` → `Option<Box<models::SessionAgentConfigParam>>`
- `CreateEnvironmentParamOpenaiHosted.desktop`: `Option<Option<models::DesktopParam>>` → `Option<Option<Box<models::DesktopParam>>>`
- `CreateEnvironmentParamOpenaiHosted.network`: `Option<Option<models::NetworkPolicyParam>>` → `Option<Option<Box<models::NetworkPolicyParam>>>`
- `CreateEnvironmentParamOpenaiHosted.packages`: `Option<Option<models::EnvironmentPackagesParam>>` → `Option<Option<Box<models::EnvironmentPackagesParam>>>`
- `CreateEnvironmentTemplateParams.desktop`: `Option<Option<models::DesktopParam>>` → `Option<Option<Box<models::DesktopParam>>>`
- `CreateEnvironmentTemplateParams.network`: `Option<Option<models::NetworkPolicyParam>>` → `Option<Option<Box<models::NetworkPolicyParam>>>`
- `CreateEnvironmentTemplateParams.packages`: `Option<Option<models::EnvironmentPackagesParam>>` → `Option<Option<Box<models::EnvironmentPackagesParam>>>`
- `CreateResponse.access_programs`: `Option<models::AccessProgramsParam>` → `Option<Box<models::AccessProgramsParam>>`
- `CreateThreadAndRunRequest.thread`: `Option<models::CreateThreadRequest>` → `Option<Box<models::CreateThreadRequest>>`
- `CreateVaultCredentialAuthParamMcpOauth.refresh`: `Option<Option<models::CreateMcpOauthRefreshParam>>` → `Option<Option<Box<models::CreateMcpOauthRefreshParam>>>`
- `DecisionResponse.usage`: `models::ResponseUsageResource` → `Box<models::ResponseUsageResource>`
- `EditImageBodyJsonParam.mask`: `Option<models::ImageRefParam>` → `Option<Box<models::ImageRefParam>>`
- `EnvironmentParamOpenaiHosted.desktop`: `Option<Option<models::DesktopParam>>` → `Option<Option<Box<models::DesktopParam>>>`
- `EnvironmentParamOpenaiHosted.network`: `Option<Option<models::NetworkPolicyParam>>` → `Option<Option<Box<models::NetworkPolicyParam>>>`
- `EnvironmentParamOpenaiHosted.packages`: `Option<Option<models::EnvironmentPackagesParam>>` → `Option<Option<Box<models::EnvironmentPackagesParam>>>`
- `EnvironmentResourceOpenaiHosted.desktop`: `models::DesktopResource` → `Box<models::DesktopResource>`
- `EnvironmentResourceOpenaiHosted.network`: `models::NetworkPolicyResource` → `Box<models::NetworkPolicyResource>`
- `EnvironmentResourceOpenaiHosted.packages`: `models::EnvironmentPackagesResource` → `Box<models::EnvironmentPackagesResource>`
- `EnvironmentTemplateResource.desktop`: `models::DesktopResource` → `Box<models::DesktopResource>`
- `EnvironmentTemplateResource.network`: `models::NetworkPolicyResource` → `Box<models::NetworkPolicyResource>`
- `EnvironmentTemplateResource.packages`: `models::EnvironmentPackagesResource` → `Box<models::EnvironmentPackagesResource>`
- `ErrorResponse2.error`: `models::ErrorBodyResource` → `Box<models::ErrorBodyResource>`
- `ImageGenTool.input_image_mask`: `Option<models::ImageGenToolInputImageMask>` → `Option<Box<models::ImageGenToolInputImageMask>>`
- `LiveCallAcceptRequest.session`: `models::LiveCallAcceptSession` → `Box<models::LiveCallAcceptSession>`
- `LiveCallAcceptSession.audio`: `Option<models::LiveMediaSessionAudioParam>` → `Option<Box<models::LiveMediaSessionAudioParam>>`
- `LiveCreateRequest.session`: `models::LiveMediaSessionCreateParams` → `Box<models::LiveMediaSessionCreateParams>`
- `LiveCreateRequest.transport`: `models::LiveWebRtcTransport` → `Box<models::LiveWebRtcTransport>`
- `LiveCreateResponse.transport`: `models::LiveWebRtcTransport` → `Box<models::LiveWebRtcTransport>`
- `LiveDecisionCompleted.decision`: `models::DecisionResponse` → `Box<models::DecisionResponse>`
- `LiveForkRequest.session`: `Option<models::LiveMediaSessionForkParams>` → `Option<Box<models::LiveMediaSessionForkParams>>`
- `LiveForkRequest.transport`: `models::LiveWebRtcTransport` → `Box<models::LiveWebRtcTransport>`
- `LiveMediaSessionCreateParams.audio`: `Option<models::LiveMediaSessionAudioParam>` → `Option<Box<models::LiveMediaSessionAudioParam>>`
- `LiveSessionCreateRequest.session`: `models::LiveMediaSessionCreateParams` → `Box<models::LiveMediaSessionCreateParams>`
- `LiveSipTransport.trunk`: `models::LiveSipTrunk` → `Box<models::LiveSipTrunk>`
- `LiveSipTrunk.auth`: `models::LiveSipTrunkAuth` → `Box<models::LiveSipTrunkAuth>`
- `McpToolApprovalFilter.always`: `Option<models::McpToolFilter>` → `Option<Box<models::McpToolFilter>>`
- `McpToolApprovalFilter.never`: `Option<models::McpToolFilter>` → `Option<Box<models::McpToolFilter>>`
- `McpToolApprovalFilter1.always`: `Option<models::BetaMcpToolFilter>` → `Option<Box<models::BetaMcpToolFilter>>`
- `McpToolApprovalFilter1.never`: `Option<models::BetaMcpToolFilter>` → `Option<Box<models::BetaMcpToolFilter>>`
- `PersistedAgentToolConfigParamWebSearch.location`: `Option<Option<models::WebSearchLocationParam>>` → `Option<Option<Box<models::WebSearchLocationParam>>>`
- `PersistedAgentToolResourceWebSearch.location`: `Option<models::WebSearchLocationResource>` → `Option<Box<models::WebSearchLocationResource>>`
- `ResponseUsageResource.input_tokens_details`: `models::InputTokensDetailsResource` → `Box<models::InputTokensDetailsResource>`
- `ResponseUsageResource.output_tokens_details`: `models::OutputTokensDetailsResource` → `Box<models::OutputTokensDetailsResource>`
- `ResponsesClientEventResponseCreate.access_programs`: `Option<models::AccessProgramsParam>` → `Option<Box<models::AccessProgramsParam>>`
- `RotateVaultCredentialAuthParamMcpOauth.refresh`: `Option<Option<models::RotateMcpOauthRefreshParam>>` → `Option<Option<Box<models::RotateMcpOauthRefreshParam>>>`
- `SessionAgentConfigParam.multi_agent`: `Option<Option<models::MultiAgentConfigCurrentParam>>` → `Option<Option<Box<models::MultiAgentConfigCurrentParam>>>`
- `SessionAgentConfigParam.reasoning`: `Option<Option<models::ReasoningParam>>` → `Option<Option<Box<models::ReasoningParam>>>`
- `SessionAgentConfigParam.text`: `Option<Option<models::TextParam>>` → `Option<Option<Box<models::TextParam>>>`
- `SessionAgentResource.multi_agent`: `models::MultiAgentConfigResource` → `Box<models::MultiAgentConfigResource>`
- `SessionAgentResource.reasoning`: `models::ReasoningResource` → `Box<models::ReasoningResource>`
- `SessionAgentResource.text`: `models::TextResource` → `Box<models::TextResource>`
- `SessionEnvironmentStateResource.error`: `Option<models::SessionEnvironmentErrorResource>` → `Option<Box<models::SessionEnvironmentErrorResource>>`
- `SessionEventAgentSessionCreated.session`: `models::SessionResource` → `Box<models::SessionResource>`
- `SessionEventAgentSessionEnvironmentConnected.environment`: `models::SessionEnvironmentStateResource` → `Box<models::SessionEnvironmentStateResource>`
- `SessionEventAgentSessionEnvironmentDisconnected.environment`: `models::SessionEnvironmentStateResource` → `Box<models::SessionEnvironmentStateResource>`
- `SessionEventAgentSessionEnvironmentExpired.environment`: `models::SessionEnvironmentStateResource` → `Box<models::SessionEnvironmentStateResource>`
- `SessionEventAgentSessionEnvironmentFailed.environment`: `models::SessionEnvironmentStateResource` → `Box<models::SessionEnvironmentStateResource>`
- `SessionEventAgentSessionEnvironmentPending.environment`: `models::SessionEnvironmentStateResource` → `Box<models::SessionEnvironmentStateResource>`
- `SessionEventAgentSessionEnvironmentReady.environment`: `models::SessionEnvironmentStateResource` → `Box<models::SessionEnvironmentStateResource>`
- `SessionEventAgentSessionEnvironmentSuspended.environment`: `models::SessionEnvironmentStateResource` → `Box<models::SessionEnvironmentStateResource>`
- `SessionEventAgentSessionFailed.session`: `models::SessionResource` → `Box<models::SessionResource>`
- `SessionEventAgentSessionIdle.session`: `models::SessionResource` → `Box<models::SessionResource>`
- `SessionEventAgentSessionInProgress.session`: `models::SessionResource` → `Box<models::SessionResource>`
- `SessionEventAgentSessionRequiresAction.session`: `models::SessionResource` → `Box<models::SessionResource>`
- `SessionEventAgentSessionSubagentActive.subagent`: `models::SubagentResource` → `Box<models::SubagentResource>`
- `SessionEventAgentSessionSubagentClosed.subagent`: `models::SubagentResource` → `Box<models::SubagentResource>`
- `SessionEventAgentSessionSubagentCreated.subagent`: `models::SubagentResource` → `Box<models::SubagentResource>`
- `SessionEventAgentSessionTurnCancelled.turn`: `models::TurnResource` → `Box<models::TurnResource>`
- `SessionEventAgentSessionTurnCancelled.usage`: `Option<models::TokenUsageResource>` → `Option<Box<models::TokenUsageResource>>`
- `SessionEventAgentSessionTurnCompleted.turn`: `models::TurnResource` → `Box<models::TurnResource>`
- `SessionEventAgentSessionTurnCompleted.usage`: `Option<models::TokenUsageResource>` → `Option<Box<models::TokenUsageResource>>`
- `SessionEventAgentSessionTurnContentPartAdded.part`: `models::OutputTextResource` → `Box<models::OutputTextResource>`
- `SessionEventAgentSessionTurnContentPartDone.part`: `models::OutputTextResource` → `Box<models::OutputTextResource>`
- `SessionEventAgentSessionTurnCreated.turn`: `models::TurnResource` → `Box<models::TurnResource>`
- `SessionEventAgentSessionTurnFailed.turn`: `models::TurnResource` → `Box<models::TurnResource>`
- `SessionEventAgentSessionTurnFailed.usage`: `Option<models::TokenUsageResource>` → `Option<Box<models::TokenUsageResource>>`
- `SessionEventAgentSessionTurnInProgress.turn`: `models::TurnResource` → `Box<models::TurnResource>`
- `SessionEventAgentSessionTurnReasoningSummaryPartAdded.part`: `models::SummaryTextResource` → `Box<models::SummaryTextResource>`
- `SessionEventAgentSessionTurnReasoningSummaryPartDone.part`: `models::SummaryTextResource` → `Box<models::SummaryTextResource>`
- `SessionEventError.error`: `models::SessionErrorResource` → `Box<models::SessionErrorResource>`
- `SessionResource.agent`: `models::SessionAgentResource` → `Box<models::SessionAgentResource>`
- `SessionResource.usage`: `Option<models::TokenUsageResource>` → `Option<Box<models::TokenUsageResource>>`
- `StaticChunkingStrategy.static`: `models::StaticChunkingStrategyStatic` → `Box<models::StaticChunkingStrategyStatic>`
- `StaticChunkingStrategyRequestParam.static`: `models::StaticChunkingStrategy` → `Box<models::StaticChunkingStrategy>`
- `StaticChunkingStrategyResponseParam.static`: `models::StaticChunkingStrategy` → `Box<models::StaticChunkingStrategy>`
- `TokenUsageResource.input_tokens_details`: `models::InputTokensDetailsResource2` → `Box<models::InputTokensDetailsResource2>`
- `TokenUsageResource.output_tokens_details`: `models::OutputTokensDetailsResource2` → `Box<models::OutputTokensDetailsResource2>`
- `TurnResource.error`: `Option<models::SessionTurnErrorResource>` → `Option<Box<models::SessionTurnErrorResource>>`
- `TurnResource.usage`: `Option<models::TokenUsageResource>` → `Option<Box<models::TokenUsageResource>>`
- `UpdateAgentParams.multi_agent`: `Option<Option<models::MultiAgentConfigCurrentParam>>` → `Option<Option<Box<models::MultiAgentConfigCurrentParam>>>`
- `UpdateAgentParams.reasoning`: `Option<Option<models::ReasoningParam>>` → `Option<Option<Box<models::ReasoningParam>>>`
- `UpdateAgentParams.text`: `Option<Option<models::TextParam>>` → `Option<Option<Box<models::TextParam>>>`
- `UpdateAgentSessionParams.agent`: `Option<models::UpdateSessionAgentParam>` → `Option<Box<models::UpdateSessionAgentParam>>`
- `UpdateEnvironmentTemplateParams.desktop`: `Option<Option<models::DesktopParam>>` → `Option<Option<Box<models::DesktopParam>>>`
- `UpdateEnvironmentTemplateParams.network`: `Option<Option<models::NetworkPolicyParam>>` → `Option<Option<Box<models::NetworkPolicyParam>>>`
- `UpdateEnvironmentTemplateParams.packages`: `Option<Option<models::EnvironmentPackagesParam>>` → `Option<Option<Box<models::EnvironmentPackagesParam>>>`
- `UpdateSessionAgentParam.reasoning`: `Option<models::UpdateSessionReasoningParam>` → `Option<Box<models::UpdateSessionReasoningParam>>`
- `VaultCredentialAuthResourceMcpOauth.refresh`: `Option<models::McpOauthRefreshResource>` → `Option<Box<models::McpOauthRefreshResource>>`
- `VectorStoreSearchRequest.ranking_options`: `Option<models::VectorStoreSearchRequestRankingOptions>` → `Option<Box<models::VectorStoreSearchRequestRankingOptions>>`
- `WebhookAgentEnvironmentExpired.data`: `models::AgentEnvironmentEvent` → `Box<models::AgentEnvironmentEvent>`
- `WebhookAgentEnvironmentFailed.data`: `models::AgentEnvironmentEvent` → `Box<models::AgentEnvironmentEvent>`
- `WebhookAgentEnvironmentReady.data`: `models::AgentEnvironmentEvent` → `Box<models::AgentEnvironmentEvent>`
- `WebhookAgentEnvironmentSuspended.data`: `models::AgentEnvironmentEvent` → `Box<models::AgentEnvironmentEvent>`
- `WebhookAgentSessionActionRequired.data`: `models::AgentSessionActionRequiredPayloadResource` → `Box<models::AgentSessionActionRequiredPayloadResource>`
- `WebhookAgentSessionCreated.data`: `models::AgentSessionCreatedPayloadResource` → `Box<models::AgentSessionCreatedPayloadResource>`
- `WebhookAgentSessionFailed.data`: `models::AgentSessionEnvironmentPayloadResource` → `Box<models::AgentSessionEnvironmentPayloadResource>`
- `WebhookAgentSessionIdle.data`: `models::AgentSessionEnvironmentPayloadResource` → `Box<models::AgentSessionEnvironmentPayloadResource>`
- `WebhookAgentSessionInProgress.data`: `models::AgentSessionEnvironmentPayloadResource` → `Box<models::AgentSessionEnvironmentPayloadResource>`
- `WebhookSafetyAlertCreated.data`: `models::WebhookSafetyAlertCreatedData` → `Box<models::WebhookSafetyAlertCreatedData>`
- `WebhookSafetyDeactivationIssued.data`: `models::WebhookSafetyDeactivationIssuedData` → `Box<models::WebhookSafetyDeactivationIssuedData>`
- `WebhookSafetyOrgAlertCreated.data`: `models::WebhookSafetyAlertCreatedData` → `Box<models::WebhookSafetyAlertCreatedData>`
- `WebhookSafetyWarningIssued.data`: `models::WebhookSafetyDeactivationIssuedData` → `Box<models::WebhookSafetyDeactivationIssuedData>`

</details>

<details><summary>nullable now honoured: one more `Option` (`type: [T, null]` / `anyOf [T, null]` in the spec) (46)</summary>

- `AgentToolConfigParamMcp.allowed_tools`: `Option<Vec<String>>` → `Option<Option<Vec<String>>>`
- `AgentToolConfigParamMcp.credential_id`: `Option<String>` → `Option<Option<String>>`
- `AgentToolConfigParamMcp.request_metadata`: `Option<std::collections::HashMap<String, serde_json::Value>>` → `Option<Option<std::collections::HashMap<String, serde_json::Value>>>`
- `AgentToolResourceMcp.allowed_tools`: `Vec<String>` → `Option<Vec<String>>`
- `AgentToolResourceMcp.credential_id`: `String` → `Option<String>`
- `Batch.errors`: `Option<Box<models::BatchErrors>>` → `Option<Option<Box<models::BatchErrors>>>`
- `BetaFunctionToolParam.parameters`: `Option<serde_json::Value>` → `Option<Option<serde_json::Value>>`
- `BetaToolSearchOutputFunctionToolParam.parameters`: `Option<serde_json::Value>` → `Option<Option<serde_json::Value>>`
- `BetaToolSearchToolParam.parameters`: `Option<serde_json::Value>` → `Option<Option<serde_json::Value>>`
- `ChatCompletionResponseMessage.function_call`: `Option<Box<models::ChatCompletionResponseMessageFunctionCall>>` → `Option<Option<Box<models::ChatCompletionResponseMessageFunctionCall>>>`
- `ChatCompletionResponseMessage.tool_calls`: `Option<Vec<models::ChatCompletionMessageToolCallsInner>>` → `Option<Option<Vec<models::ChatCompletionMessageToolCallsInner>>>`
- `CreateAgentParams.metadata`: `Option<std::collections::HashMap<String, String>>` → `Option<Option<std::collections::HashMap<String, String>>>`
- `CreateAgentSessionParams.metadata`: `Option<std::collections::HashMap<String, String>>` → `Option<Option<std::collections::HashMap<String, String>>>`
- `CreateChatCompletionRequest.audio`: `Option<Box<models::CreateChatCompletionRequestAllOfAudio>>` → `Option<Option<Box<models::CreateChatCompletionRequestAllOfAudio>>>`
- `CreateChatCompletionRequest.logit_bias`: `Option<std::collections::HashMap<String, i32>>` → `Option<Option<std::collections::HashMap<String, i32>>>`
- `CreateChatCompletionStreamResponseChoicesInner.logprobs`: `Option<Box<models::CreateChatCompletionStreamResponseChoicesInnerLogprobs>>` → `Option<Option<Box<models::CreateChatCompletionStreamResponseChoicesInnerLogprobs>>>`
- `CreateCompletionRequest.logit_bias`: `Option<std::collections::HashMap<String, i32>>` → `Option<Option<std::collections::HashMap<String, i32>>>`
- `CreateEnvironmentParamOpenaiHosted.env`: `Option<std::collections::HashMap<String, String>>` → `Option<Option<std::collections::HashMap<String, String>>>`
- `CreateEnvironmentTemplateParams.env`: `Option<std::collections::HashMap<String, String>>` → `Option<Option<std::collections::HashMap<String, String>>>`
- `CreateThreadAndRunRequest.tool_resources`: `Option<Box<models::CreateThreadAndRunRequestToolResources>>` → `Option<Option<Box<models::CreateThreadAndRunRequestToolResources>>>`
- `CreateVaultParams.metadata`: `Option<std::collections::HashMap<String, String>>` → `Option<Option<std::collections::HashMap<String, String>>>`
- `DecisionRequest.prompt_cache_key`: `Option<String>` → `Option<Option<String>>`
- `DecisionRequest.safety_identifier`: `Option<String>` → `Option<Option<String>>`
- `EnvironmentParamOpenaiHosted.env`: `Option<std::collections::HashMap<String, String>>` → `Option<Option<std::collections::HashMap<String, String>>>`
- `FineTuningJob.hyperparameters`: `Option<Box<models::FineTuningJobHyperparameters>>` → `Option<Option<Box<models::FineTuningJobHyperparameters>>>`
- `FunctionToolParam.parameters`: `Option<serde_json::Value>` → `Option<Option<serde_json::Value>>`
- `ListAgentSessionSubagents200Response.first_id`: `String` → `Option<String>`
- `ListAgentSessionSubagents200Response.last_id`: `String` → `Option<String>`
- `McpTransportConfigParamHttp.headers`: `Option<std::collections::HashMap<String, String>>` → `Option<Option<std::collections::HashMap<String, String>>>`
- `McpTransportConfigParamStdio.env`: `Option<std::collections::HashMap<String, String>>` → `Option<Option<std::collections::HashMap<String, String>>>`
- `PersistedAgentToolConfigParamMcp.allowed_tools`: `Option<Vec<String>>` → `Option<Option<Vec<String>>>`
- `PersistedAgentToolConfigParamMcp.credential_id`: `Option<String>` → `Option<Option<String>>`
- `PersistedAgentToolConfigParamMcp.request_metadata`: `Option<std::collections::HashMap<String, serde_json::Value>>` → `Option<Option<std::collections::HashMap<String, serde_json::Value>>>`
- `PersistedAgentToolResourceMcp.allowed_tools`: `Vec<String>` → `Option<Vec<String>>`
- `PersistedAgentToolResourceMcp.credential_id`: `String` → `Option<String>`
- `PersistedMcpTransportConfigParamHttp.headers`: `Option<std::collections::HashMap<String, String>>` → `Option<Option<std::collections::HashMap<String, String>>>`
- `RealtimeSessionCreateResponseAudioInput.noise_reduction`: `Option<Box<models::RealtimeSessionCreateResponseAudioInputNoiseReduction>>` → `Option<Option<Box<models::RealtimeSessionCreateResponseAudioInputNoiseReduction>>>`
- `RealtimeSessionCreateResponseAudioInput.turn_detection`: `Option<Box<models::RealtimeSessionCreateResponseAudioInputTurnDetection>>` → `Option<Option<Box<models::RealtimeSessionCreateResponseAudioInputTurnDetection>>>`
- `RealtimeSessionCreateResponseGaAudioInput.noise_reduction`: `Option<Box<models::RealtimeSessionCreateResponseGaAudioInputNoiseReduction>>` → `Option<Option<Box<models::RealtimeSessionCreateResponseGaAudioInputNoiseReduction>>>`
- `RealtimeTranscriptionSessionCreateResponseGaAudioInput.noise_reduction`: `Option<Box<models::RealtimeSessionCreateResponseAudioInputNoiseReduction>>` → `Option<Option<Box<models::RealtimeSessionCreateResponseAudioInputNoiseReduction>>>`
- `ToolSearchOutputFunctionToolParam.parameters`: `Option<serde_json::Value>` → `Option<Option<serde_json::Value>>`
- `ToolSearchToolParam.parameters`: `Option<serde_json::Value>` → `Option<Option<serde_json::Value>>`
- `UpdateAgentParams.metadata`: `Option<std::collections::HashMap<String, String>>` → `Option<Option<std::collections::HashMap<String, String>>>`
- `UpdateAgentSessionParams.metadata`: `Option<std::collections::HashMap<String, String>>` → `Option<Option<std::collections::HashMap<String, String>>>`
- `UpdateEnvironmentTemplateParams.env`: `Option<std::collections::HashMap<String, String>>` → `Option<Option<std::collections::HashMap<String, String>>>`
- `WebSearch.user_location`: `Option<Box<models::WebSearchUserLocation>>` → `Option<Option<Box<models::WebSearchUserLocation>>>`

</details>

<details><summary>other type change (41)</summary>

- `AgentToolConfigParamMcp.connection_origin`: `Option<Box<models::AgentToolConfigParamMcpConnectionOrigin>>` → `Option<Option<models::McpConnectionOriginParam>>`
- `BetaResponse.incomplete_details`: `Option<Box<models::Object021>>` → `Option<Box<models::Object019>>`
- `BetaWebSearchTool.filters`: `Option<Option<Box<models::Object028>>>` → `Option<Option<Box<models::Object026>>>`
- `ChatkitWorkflow.state_variables`: `Option<std::collections::HashMap<String, models::Object029Value>>` → `Option<std::collections::HashMap<String, models::Object027Value>>`
- `CreateAgentEnvironmentParams.environment`: `models::CreateEnvironmentParamOpenaiHosted` → `Box<models::CreateEnvironmentParam>`
- `CreateAssistantRequest.description`: `Option<Box<models::CreateAssistantRequestDescription>>` → `Option<Option<String>>`
- `CreateAssistantRequest.instructions`: `Option<Box<models::CreateAssistantRequestInstructions>>` → `Option<Option<String>>`
- `CreateAssistantRequest.name`: `Option<Box<models::CreateAssistantRequestName>>` → `Option<Option<String>>`
- `CreateAssistantRequest.response_format`: `Option<Box<models::CreateAssistantRequestResponseFormat>>` → `Option<Option<Box<models::AssistantsApiResponseFormatOption>>>`
- `CreateAssistantRequest.temperature`: `Option<Box<models::CreateAssistantRequestTemperature>>` → `Option<Option<f64>>`
- `CreateAssistantRequest.tool_resources`: `Option<Box<models::CreateAssistantRequestToolResources>>` → `Option<Option<Box<models::Object07>>>`
- `CreateAssistantRequest.top_p`: `Option<Box<models::CreateAssistantRequestTopP>>` → `Option<Option<f64>>`
- `CreateEnvironmentParamOpenaiHosted.plugins`: `Option<Option<Vec<models::HostedPluginParamInline>>>` → `Option<Option<Vec<models::HostedPluginParam>>>`
- `CreateEnvironmentTemplateParams.plugins`: `Option<Option<Vec<models::HostedPluginParamInline>>>` → `Option<Option<Vec<models::HostedPluginParam>>>`
- `CreateThreadAndRunRequestToolResources.code_interpreter`: `Option<Box<models::CreateThreadAndRunRequestToolResourcesCodeInterpreter>>` → `Option<Box<models::Object07CodeInterpreter>>`
- `CreateThreadRequest.tool_resources`: `Option<Box<models::CreateThreadRequestToolResources>>` → `Option<Option<Box<models::Object010>>>`
- `DecisionInputImage.detail`: `Option<Box<models::DecisionInputImageDetail>>` → `Option<Option<models::ImageDetailParam>>`
- `DecisionInputImage.prompt_cache_breakpoint`: `Option<models::PromptCacheBreakpoint1>` → `Option<Option<Box<models::PromptCacheBreakpoint>>>`
- `DecisionInputText.prompt_cache_breakpoint`: `Option<models::PromptCacheBreakpoint>` → `Option<Option<Box<models::PromptCacheBreakpoint>>>`
- `DecisionRequest.prompt_cache_options`: `Option<models::DecisionRequestPromptCacheOptions>` → `Option<Option<Box<models::DecisionRequestPromptCacheOptions>>>`
- `EnvironmentParamOpenaiHosted.plugins`: `Option<Option<Vec<models::HostedPluginParamInline>>>` → `Option<Option<Vec<models::HostedPluginParam>>>`
- `EnvironmentResourceOpenaiHosted.plugins`: `Vec<models::HostedPluginResourceInline>` → `Vec<models::HostedPluginResource>`
- `EnvironmentTemplateResource.plugins`: `Vec<models::HostedPluginResourceInline>` → `Vec<models::HostedPluginResource>`
- `EvalRunOutputItemResult.sample`: `Option<Box<models::EvalRunOutputItemResultSample>>` → `Option<Option<std::collections::HashMap<String, serde_json::Value>>>`
- `HostedPluginParamInline.source`: `models::InlineCapabilitySourceParamBase64` → `Box<models::InlineCapabilitySourceParam>`
- `HostedSkillParamInline.source`: `models::InlineCapabilitySourceParamBase64` → `Box<models::InlineCapabilitySourceParam>`
- `LiveDecisionEvent.source_event_type`: `Option<Box<models::LiveDecisionEventSourceEventType>>` → `Option<SourceEventType>`
- `Object015.code_interpreter`: `Option<Box<models::CreateThreadAndRunRequestToolResourcesCodeInterpreter>>` → `Option<Box<models::Object07CodeInterpreter>>`
- `Object024.data`: `Box<models::RunStepObject>` → `Vec<models::Object024DataInner>`
- `PersistedAgentToolConfigParamMcp.connection_origin`: `Option<Box<models::PersistedAgentToolConfigParamMcpConnectionOrigin>>` → `Option<Option<models::McpConnectionOriginParam>>`
- `PublicEnvironmentResource.plugins`: `Vec<models::HostedPluginResourceInline>` → `Vec<models::HostedPluginResource>`
- `RealtimeSessionCreateResponse.turn_detection`: `Option<Box<models::RealtimeSessionCreateRequestTurnDetection>>` → `Option<Option<Box<models::RealtimeSessionCreateResponseTurnDetection>>>`
- `RealtimeTranslationSessionCreateRequest.audio`: `Option<Box<models::RealtimeTranslationSessionCreateRequestAudio>>` → `Option<Box<models::RealtimeTranslationSessionAudio>>`
- `RealtimeTranslationSessionUpdateRequest.audio`: `Option<Box<models::RealtimeTranslationSessionCreateRequestAudio>>` → `Option<Box<models::RealtimeTranslationSessionAudio>>`
- `Response.incomplete_details`: `Option<Box<models::Object021>>` → `Option<Box<models::Object019>>`
- `RunGraderResponseMetadata.token_usage`: `Option<Box<models::Object022>>` → `Option<Box<models::Object020>>`
- `RunStepObject.last_error`: `Option<Box<models::Object023>>` → `Option<Box<models::Object021>>`
- `UpdateEnvironmentTemplateParams.plugins`: `Option<Option<Vec<models::HostedPluginParamInline>>>` → `Option<Option<Vec<models::HostedPluginParam>>>`
- `User.projects`: `Option<Option<Box<models::Object026>>>` → `Option<Option<Box<models::Object024>>>`
- `VectorStoreFileObject.last_error`: `Option<Box<models::Object027>>` → `Option<Box<models::Object025>>`
- `WebSearchTool.filters`: `Option<Option<Box<models::Object028>>>` → `Option<Option<Box<models::Object026>>>`

</details>

### API functions with changed signatures (2)

- `apis/default_api.rs`: `organization_decisionsusage`
- `apis/vaults_api.rs`: `list_vault_credentials`

### Enum variant changes (45)

- `AgentToolConfigParamMcpConnectionOrigin` (models/agent_tool_config_param_mcp_connection_origin.rs): +[] -['Mcpconnectionoriginparam', 'Null']
- `CreateAssistantRequestDescription` (models/create_assistant_request_description.rs): +[] -['Null', 'Text']
- `CreateAssistantRequestInstructions` (models/create_assistant_request_instructions.rs): +[] -['Null', 'Text']
- `CreateAssistantRequestName` (models/create_assistant_request_name.rs): +[] -['Null', 'Text']
- `CreateAssistantRequestResponseFormat` (models/create_assistant_request_response_format.rs): +[] -['Assistantsapiresponseformatoption', 'Null']
- `CreateAssistantRequestTemperature` (models/create_assistant_request_temperature.rs): +[] -['Null']
- `CreateAssistantRequestToolResources` (models/create_assistant_request_tool_resources.rs): +[] -['Null']
- `CreateAssistantRequestTopP` (models/create_assistant_request_top_p.rs): +[] -['Null']
- `Type` (models/create_environment_param.rs): +['OpenaiHosted'] -[]
- `CreateModerationRequestInputOneOfInner` (models/create_moderation_request_input_one_of_inner.rs): +['Object027'] -['Object029']
- `CreateThreadRequestToolResources` (models/create_thread_request_tool_resources.rs): +[] -['Null']
- `DecisionInputImageDetail` (models/decision_input_image_detail.rs): +[] -['Imagedetailparam', 'Null']
- `EvalRunOutputItemResultSample` (models/eval_run_output_item_result_sample.rs): +[] -['Null']
- `Weight` (models/fine_tune_chat_completion_request_assistant_message.rs): +[] -['Variant0', 'Variant1']
- `Type` (models/hosted_plugin_param.rs): +['Inline'] -[]
- `Type` (models/hosted_plugin_resource.rs): +['Inline'] -[]
- `MediaType` (models/inline_capability_source_param.rs): +['ApplicationSlashZip'] -[]
- `Type` (models/inline_capability_source_param.rs): +['Base64'] -[]
- `SourceEventType` (models/live_decision_event.rs): +['SessionInputTranscriptDelta', 'SessionOutputTranscriptDelta'] -[]
- `LiveDecisionEventSourceEventType` (models/live_decision_event_source_event_type.rs): +[] -['Null', 'TextVariant']
- `LiveDecisionEventSourceEventTypeTextVariantEnum` (models/live_decision_event_source_event_type.rs): +[] -['SessionOutputTranscriptDelta']
- `SourceEventType` (models/live_server_event.rs): +['SessionInputTranscriptDelta', 'SessionOutputTranscriptDelta'] -[]
- `SourceEventType` (models/live_server_event2.rs): +['SessionInputTranscriptDelta', 'SessionOutputTranscriptDelta'] -[]
- `Rate` (models/live_session_audio_format_pcm_param.rs): +[] -['Variant16000', 'Variant24000']
- `SourceEventType` (models/live_sideband_server_event.rs): +['SessionInputTranscriptDelta', 'SessionOutputTranscriptDelta'] -[]
- `Reason` (models/object0_19.rs): +['ContentFilter', 'MaxMessages', 'MaxOutputTokens', 'Steered'] -[]
- `Code` (models/object0_21.rs): +['RateLimitExceeded', 'ServerError'] -[]
- `Reason` (models/object0_21.rs): +[] -['ContentFilter', 'MaxMessages', 'MaxOutputTokens', 'Steered']
- `Event` (models/object0_22.rs): +['ThreadRunStepCreated'] -[]
- `Code` (models/object0_23.rs): +[] -['RateLimitExceeded', 'ServerError']
- `Event` (models/object0_23.rs): +['ThreadRunCreated'] -[]
- `Event` (models/object0_24.rs): +[] -['ThreadRunStepCreated']
- `Object` (models/object0_24.rs): +['List'] -[]
- `Code` (models/object0_25.rs): +['InvalidFile', 'ServerError', 'UnsupportedFile'] -[]
- `Event` (models/object0_25.rs): +[] -['ThreadRunCreated']
- `Object` (models/object0_26.rs): +[] -['List']
- `Code` (models/object0_27.rs): +[] -['InvalidFile', 'ServerError', 'UnsupportedFile']
- `Type` (models/object0_27.rs): +['ImageUrl'] -[]
- `Type` (models/object0_29.rs): +[] -['ImageUrl']
- `Rate` (models/pcm_audio_format.rs): +[] -['Variant24000']
- `PersistedAgentToolConfigParamMcpConnectionOrigin` (models/persisted_agent_tool_config_param_mcp_connection_origin.rs): +[] -['Mcpconnectionoriginparam', 'Null']
- `Mode` (models/prompt_cache_breakpoint_1.rs): +[] -['Explicit']
- `RunStepStreamEvent` (models/run_step_stream_event.rs): +['Object022'] -['Object024']
- `RunStreamEvent` (models/run_stream_event.rs): +['Object023'] -['Object025']
- `TracingConfiguration1` (models/tracing_configuration_1.rs): +['TracingConfiguration'] -['TracingConfiguration1']

### Derive changes (52)

- `AgentToolConfigParamMcp`: +['Default'] -[]
- `CreateAssistantRequest`: +['Default'] -[]
- `DecisionInputImage`: +['Default'] -[]
- `Weight`: +['Deserialize_repr', 'Serialize_repr'] -['Deserialize', 'Serialize']
- `Rate`: +['Deserialize_repr', 'Serialize_repr'] -['Deserialize', 'Serialize']
- `Object022`: +[] -['Default']
- `Object023`: +[] -['Default']
- `Object025`: +['Default'] -[]
- `Object026`: +['Default'] -[]
- `Rate`: +['Deserialize_repr', 'Serialize_repr'] -['Deserialize', 'Serialize']
- `PersistedAgentToolConfigParamMcp`: +['Default'] -[]

## Migrating

- A field that gained an `Option` was already nullable in the spec
  (`type: [T, "null"]` or `anyOf: [T, null]`); 7.15.0 dropped the `null`.
  Required nullable fields are now `Option<T>`, optional nullable fields
  `Option<Option<T>>` (serialized with `serde_with::rust::double_option`:
  `None` omits the key, `Some(None)` sends `null`).
  An explicit `null` in a response therefore deserializes as `Some(None)` where
  0.16 collapsed it into `None`.
- A field that gained a `Box` is a required or optional reference to another
  model; `new()` constructors still take the unboxed value. Use `*field` or
  `field.as_ref()` where you read it.
- `serde_json::Value` fields that became `HashMap<String, serde_json::Value>`
  were free-form objects in the spec (`type: object` without properties).
- Types such as `CreateAssistantRequestDescription` were wrappers 7.15.0
  generated for nullable primitives; the field is now the primitive inside
  `Option<Option<..>>`.
- `Object0NN*` names are generator-assigned for untitled inline objects and
  were renumbered; match on the field that holds them rather than the name.
