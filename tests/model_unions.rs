//! Regression tests for the unions rendered with the generator's `modelUnions` option. Samples follow the spec's
//! schemas; each must decode and serialize back to the same JSON.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

use openai_client_base::apis::{configuration::Configuration, images_api, videos_api};
use openai_client_base::models;
use serde::{de::DeserializeOwned, Serialize};

fn round_trips<T: DeserializeOwned + Serialize>(json: &str) -> T {
    let value: T = serde_json::from_str(json).unwrap_or_else(|e| panic!("{json}: {e}"));
    assert_eq!(
        serde_json::to_value(&value).unwrap(),
        serde_json::from_str::<serde_json::Value>(json).unwrap(),
        "{json}"
    );
    value
}

/// Duplicate tag on the wire and no decode: every branch decodes, and the tag goes out once.
fn tag_once<T: DeserializeOwned + Serialize>(tag: &str, json: &str) {
    let value: T = round_trips(json);
    let text = serde_json::to_string(&value).unwrap();
    assert_eq!(text.matches(&format!("\"{tag}\":")).count(), 1, "{text}");
}

#[test]
fn duplicate_tag_is_gone() {
    tag_once::<models::ChatCompletionRequestMessage>(
        "role",
        r#####"{"content": "x", "role": "developer"}"#####,
    );
    tag_once::<models::ChatCompletionRequestMessage>(
        "role",
        r#####"{"content": "x", "role": "system"}"#####,
    );
    tag_once::<models::ChatCompletionRequestMessage>(
        "role",
        r#####"{"content": "x", "role": "user"}"#####,
    );
    tag_once::<models::ChatCompletionRequestMessage>("role", r#####"{"role": "assistant"}"#####);
    tag_once::<models::ChatCompletionRequestMessage>(
        "role",
        r#####"{"content": "x", "role": "tool", "tool_call_id": "x"}"#####,
    );
    tag_once::<models::ChatCompletionRequestMessage>(
        "role",
        r#####"{"content": "x", "name": "x", "role": "function"}"#####,
    );
    tag_once::<models::UsageTimeBucketResultsInner>(
        "object",
        r#####"{"input_tokens": 1, "num_model_requests": 1, "object": "organization.usage.completions.result", "output_tokens": 1}"#####,
    );
    tag_once::<models::UsageTimeBucketResultsInner>(
        "object",
        r#####"{"input_tokens": 1, "num_model_requests": 1, "object": "organization.usage.embeddings.result"}"#####,
    );
    tag_once::<models::UsageTimeBucketResultsInner>(
        "object",
        r#####"{"input_tokens": 1, "num_model_requests": 1, "object": "organization.usage.moderations.result"}"#####,
    );
    tag_once::<models::UsageTimeBucketResultsInner>(
        "object",
        r#####"{"images": 1, "num_model_requests": 1, "object": "organization.usage.images.result"}"#####,
    );
    tag_once::<models::UsageTimeBucketResultsInner>(
        "object",
        r#####"{"characters": 1, "num_model_requests": 1, "object": "organization.usage.audio_speeches.result"}"#####,
    );
    tag_once::<models::UsageTimeBucketResultsInner>(
        "object",
        r#####"{"num_model_requests": 1, "object": "organization.usage.audio_transcriptions.result", "seconds": 1}"#####,
    );
    tag_once::<models::UsageTimeBucketResultsInner>(
        "object",
        r#####"{"object": "organization.usage.vector_stores.result", "usage_bytes": 1}"#####,
    );
    tag_once::<models::UsageTimeBucketResultsInner>(
        "object",
        r#####"{"num_sessions": 1, "object": "organization.usage.code_interpreter_sessions.result"}"#####,
    );
    tag_once::<models::UsageTimeBucketResultsInner>(
        "object",
        r#####"{"num_requests": 1, "object": "organization.usage.file_searches.result"}"#####,
    );
    tag_once::<models::UsageTimeBucketResultsInner>(
        "object",
        r#####"{"num_model_requests": 1, "num_requests": 1, "object": "organization.usage.web_searches.result"}"#####,
    );
    tag_once::<models::UsageTimeBucketResultsInner>(
        "object",
        r#####"{"object": "organization.costs.result"}"#####,
    );
    tag_once::<models::ComputerUseApprovalResponseKindResource>(
        "action",
        r#####"{"action": "submit", "selected_option": "x", "type": "browser_authentication"}"#####,
    );
    tag_once::<models::ComputerUseApprovalResponseKindResource>(
        "action",
        r#####"{"action": "cancel", "type": "browser_authentication"}"#####,
    );
}

#[test]
fn a_message_decodes_by_its_role() {
    assert!(matches!(
        serde_json::from_str::<models::ChatCompletionRequestMessage>(
            r#####"{"role": "user", "content": "hi"}"#####
        )
        .unwrap(),
        models::ChatCompletionRequestMessage::ChatCompletionRequestUserMessage(_)
    ));
    assert!(matches!(
        serde_json::from_str::<models::ChatCompletionRequestMessage>(
            r#####"{"role": "system", "content": "hi"}"#####
        )
        .unwrap(),
        models::ChatCompletionRequestMessage::ChatCompletionRequestSystemMessage(_)
    ));
    assert!(matches!(
        serde_json::from_str::<models::ChatCompletionRequestMessage>(
            r#####"{"role": "assistant", "content": "hi"}"#####
        )
        .unwrap(),
        models::ChatCompletionRequestMessage::ChatCompletionRequestAssistantMessage(_)
    ));
}

#[test]
fn a_union_does_not_hold_itself() {
    round_trips::<models::AuditLogActor>(
        r#####"{"type": "session", "session": {"ip_address": "127.0.0.1"}}"#####,
    );
    round_trips::<models::AuditLogActor>(
        r#####"{"type": "api_key", "api_key": {"id": "key_1", "type": "user"}}"#####,
    );
    round_trips::<models::McpToolCallError>(
        r#####"{"code": 1, "message": "x", "type": "mcp_protocol_error"}"#####,
    );
    round_trips::<models::McpToolCallError>(
        r#####"{"content": "x", "type": "mcp_tool_execution_error"}"#####,
    );
    round_trips::<models::McpToolCallError>(
        r#####"{"code": 1, "message": "x", "type": "http_error"}"#####,
    );
    round_trips::<models::BetaMcpToolCallError>(
        r#####"{"code": 1, "message": "x", "type": "mcp_protocol_error"}"#####,
    );
    round_trips::<models::BetaMcpToolCallError>(
        r#####"{"content": "x", "type": "mcp_tool_execution_error"}"#####,
    );
    round_trips::<models::BetaMcpToolCallError>(
        r#####"{"code": 1, "message": "x", "type": "http_error"}"#####,
    );
    round_trips::<models::RealtimeSessionCreateRequestTracing>(
        r#####"{"workflow_name": "flow", "group_id": "g"}"#####,
    );
    round_trips::<models::RealtimeSessionCreateRequestTracing>(r#####""auto""#####);
    round_trips::<models::RealtimeSessionCreateResponseTracing>(
        r#####"{"workflow_name": "flow", "group_id": "g"}"#####,
    );
    round_trips::<models::RealtimeSessionCreateResponseTracing>(r#####""auto""#####);
}

#[test]
fn null_next_to_an_object_keeps_the_object() {
    round_trips::<models::Prompt>(r#####"{"id": "pmpt_1", "version": "3"}"#####);
    round_trips::<models::BetaPrompt>(r#####"{"id": "pmpt_1"}"#####);
    round_trips::<models::BetaResponseStreamOptions>(r#####"{"include_obfuscation": true}"#####);
    round_trips::<models::ChatCompletionStreamOptions>(r#####"{"include_usage": true}"#####);
    round_trips::<models::ResponseStreamOptions>(r#####"{"include_obfuscation": false}"#####);
    round_trips::<models::RunCompletionUsage>(
        r#####"{"completion_tokens": 1, "prompt_tokens": 2, "total_tokens": 3}"#####,
    );
    round_trips::<models::RunStepCompletionUsage>(
        r#####"{"completion_tokens": 1, "prompt_tokens": 2, "total_tokens": 3}"#####,
    );
    round_trips::<models::FineTuningJobError>(
        r#####"{"code": "x", "message": "y", "param": null}"#####,
    );
    assert_eq!(
        serde_json::from_str::<Option<models::Prompt>>("null").unwrap(),
        None
    );
}

#[test]
fn a_discriminated_any_of_is_not_empty() {
    round_trips::<models::ResponsesServerEvent>(
        r#####"{"delta": "x", "sequence_number": 1, "type": "response.audio.delta"}"#####,
    );
    round_trips::<models::ResponsesServerEvent>(
        r#####"{"sequence_number": 1, "type": "response.audio.done"}"#####,
    );
    round_trips::<models::BetaResponsesServerEvent>(
        r#####"{"delta": "x", "sequence_number": 1, "type": "response.audio.delta"}"#####,
    );
    round_trips::<models::BetaResponsesServerEvent>(
        r#####"{"sequence_number": 1, "type": "response.audio.done"}"#####,
    );
    round_trips::<models::LiveServerEvent>(
        r#####"{"event_id": "x", "session": {"expires_at": 1, "id": "x", "model": "x", "status": "active"}, "type": "session.started"}"#####,
    );
    round_trips::<models::LiveServerEvent>(
        r#####"{"event_id": "x", "session": {"expires_at": 1, "id": "x", "model": "x", "status": "active"}, "type": "session.updated"}"#####,
    );
}

#[test]
fn every_branch_has_its_variant() {
    round_trips::<models::CreateEvalItem>(r#####"{"content": "x", "role": "user"}"#####);
    round_trips::<models::CreateEvalItem>(r#####"{"content": "x", "role": "x"}"#####);
    round_trips::<models::VoiceIdsOrCustomVoice>(r#####""x""#####);
    round_trips::<models::VoiceIdsOrCustomVoice>(r#####"{"id": "x"}"#####);
    round_trips::<models::ChoiceValueParam>(r#####""x""#####);
    round_trips::<models::ChoiceValueParam>(r#####"true"#####);
    round_trips::<models::ChoiceValueResource>(r#####""x""#####);
    round_trips::<models::ChoiceValueResource>(r#####"true"#####);
    round_trips::<models::ComparisonFilterValue>(r#####""x""#####);
    round_trips::<models::ComparisonFilterValue>(r#####"0.5"#####);
    round_trips::<models::ComparisonFilterValue>(r#####"true"#####);
    round_trips::<models::ComparisonFilterValue>(r#####"["x"]"#####);
    round_trips::<models::RealtimeSessionCreateRequestTracing>(r#####""auto""#####);
    round_trips::<models::RealtimeSessionCreateRequestTracing>(r#####"{}"#####);
    round_trips::<models::RealtimeSessionCreateResponseTracing>(r#####""auto""#####);
    round_trips::<models::RealtimeSessionCreateResponseTracing>(r#####"{}"#####);
    round_trips::<models::BetaComparisonFilterValue>(r#####""x""#####);
    round_trips::<models::BetaComparisonFilterValue>(r#####"0.5"#####);
    round_trips::<models::BetaComparisonFilterValue>(r#####"true"#####);
    round_trips::<models::BetaComparisonFilterValue>(r#####"["x"]"#####);
    round_trips::<models::FineTuneDpoHyperparametersBeta>(r#####""auto""#####);
    round_trips::<models::FineTuneDpoHyperparametersBeta>(r#####"0.5"#####);
    round_trips::<models::FineTuneDpoHyperparametersBatchSize>(r#####""auto""#####);
    round_trips::<models::FineTuneDpoHyperparametersBatchSize>(r#####"1"#####);
    round_trips::<models::FineTuneDpoHyperparametersLearningRateMultiplier>(r#####""auto""#####);
    round_trips::<models::FineTuneDpoHyperparametersLearningRateMultiplier>(r#####"0.5"#####);
    round_trips::<models::FineTuneDpoHyperparametersNEpochs>(r#####""auto""#####);
    round_trips::<models::FineTuneDpoHyperparametersNEpochs>(r#####"1"#####);
    round_trips::<models::FineTuneReinforcementHyperparametersComputeMultiplier>(
        r#####""auto""#####,
    );
    round_trips::<models::FineTuneReinforcementHyperparametersComputeMultiplier>(r#####"0.5"#####);
    round_trips::<models::FineTuneReinforcementHyperparametersEvalInterval>(r#####""auto""#####);
    round_trips::<models::FineTuneReinforcementHyperparametersEvalInterval>(r#####"1"#####);
    round_trips::<models::FineTuneReinforcementHyperparametersEvalSamples>(r#####""auto""#####);
    round_trips::<models::FineTuneReinforcementHyperparametersEvalSamples>(r#####"1"#####);
    round_trips::<models::FineTuningJobError>(
        r#####"{"code": "x", "message": "x", "param": "x"}"#####,
    );
    round_trips::<models::FineTuningJobError>(r#####"{"key": "value"}"#####);
    round_trips::<models::RealtimeBetaResponseCreateParamsMaxOutputTokens>(r#####"1"#####);
    round_trips::<models::RealtimeBetaResponseCreateParamsMaxOutputTokens>(r#####""inf""#####);
    round_trips::<models::LiveDecisionChoiceParamValue>(r#####""x""#####);
    round_trips::<models::LiveDecisionChoiceParamValue>(r#####"true"#####);
    round_trips::<models::LiveDelegationItemTarget>(r#####""client""#####);
    round_trips::<models::LiveDelegationItemTarget>(r#####""responses""#####);
    round_trips::<models::LiveDelegationItemTarget>(r#####""decisions""#####);
    round_trips::<models::LiveSessionClosedReason>(r#####""close_requested""#####);
    round_trips::<models::LiveSessionClosedReason>(r#####""expired""#####);
    round_trips::<models::LiveSessionClosedReason>(r#####""content""#####);
    round_trips::<models::LiveSessionClosedReason>(r#####""remote_hangup""#####);
    round_trips::<models::LiveSessionClosedReason>(r#####""connection_lost""#####);
}

#[test]
fn an_inline_object_keeps_its_own_fields() {
    round_trips::<models::BetaResponseError>(
        r#####"{"code": "server_error", "message": "x", "type": "x", "param": "x", "misalignment": {"review_target": "x", "error_type": "x", "detailed_explanation": "x", "steer": {"message": "x"}}}"#####,
    );
    round_trips::<models::ResponseError>(
        r#####"{"code": "server_error", "message": "x", "type": "x", "param": "x", "misalignment": {"review_target": "x", "error_type": "x", "detailed_explanation": "x", "steer": {"message": "x"}}}"#####,
    );
    round_trips::<models::BetaWebSearchApproximateLocation>(
        r#####"{"type": "approximate", "country": "x", "region": "x", "city": "x", "timezone": "x"}"#####,
    );
    round_trips::<models::WebSearchApproximateLocation>(
        r#####"{"type": "approximate", "country": "x", "region": "x", "city": "x", "timezone": "x"}"#####,
    );
    round_trips::<models::CreateEvalCompletionsRunDataSourceInputMessages>(
        r#####"{"template": [{"content": "x", "role": "user", "phase": "commentary", "type": "message"}], "type": "template"}"#####,
    );
    round_trips::<models::CreateEvalCompletionsRunDataSourceInputMessages>(
        r#####"{"item_reference": "x", "type": "item_reference"}"#####,
    );
    round_trips::<models::CreateEvalResponsesRunDataSourceInputMessages>(
        r#####"{"template": [{"content": "x", "role": "x"}], "type": "template"}"#####,
    );
    round_trips::<models::CreateEvalResponsesRunDataSourceInputMessages>(
        r#####"{"item_reference": "x", "type": "item_reference"}"#####,
    );
    round_trips::<models::RealtimeTruncation>(r#####""auto""#####);
    round_trips::<models::RealtimeTruncation>(
        r#####"{"retention_ratio": 0.5, "type": "retention_ratio", "token_limits": {"post_instructions": 0}}"#####,
    );
    round_trips::<models::RealtimeTurnDetection>(
        r#####"{"type": "server_vad", "threshold": 0.5, "prefix_padding_ms": 1, "silence_duration_ms": 1, "create_response": true, "interrupt_response": true, "idle_timeout_ms": 5000}"#####,
    );
    round_trips::<models::RealtimeTurnDetection>(
        r#####"{"type": "semantic_vad", "eagerness": "low", "create_response": true, "interrupt_response": true}"#####,
    );
    round_trips::<models::RealtimeAudioFormats>(r#####"{"type": "audio/pcm", "rate": 24000}"#####);
    round_trips::<models::RealtimeAudioFormats>(r#####"{"type": "audio/pcmu"}"#####);
    round_trips::<models::RealtimeAudioFormats>(r#####"{"type": "audio/pcma"}"#####);
}

#[test]
fn input_item_keeps_status() {
    round_trips::<models::InputItem>(
        r#####"{"type": "message", "role": "user", "status": "completed", "content": [{"type": "input_text", "text": "hi"}]}"#####,
    );
    let item = round_trips::<models::InputItem>(
        r#####"{"type": "message", "role": "user", "status": "completed", "content": [{"type": "input_text", "text": "hi"}]}"#####,
    );
    assert!(matches!(item, models::InputItem::Item(_)), "{item:?}");
    assert!(matches!(
        round_trips::<models::InputItem>(r#####"{"role": "assistant", "content": "hi"}"#####),
        models::InputItem::EasyInputMessage(_)
    ));
}

#[test]
fn the_integer_branch_is_back() {
    assert!(matches!(
        round_trips::<models::FineTuneDpoHyperparametersBatchSize>("7"),
        models::FineTuneDpoHyperparametersBatchSize::Integer(7)
    ));
    assert!(matches!(
        round_trips::<models::FineTuneDpoHyperparametersBatchSize>("\"auto\""),
        models::FineTuneDpoHyperparametersBatchSize::TextVariant(_)
    ));
    assert!(matches!(
        round_trips::<models::FineTuneDpoHyperparametersNEpochs>("7"),
        models::FineTuneDpoHyperparametersNEpochs::Integer(7)
    ));
    assert!(matches!(
        round_trips::<models::FineTuneDpoHyperparametersNEpochs>("\"auto\""),
        models::FineTuneDpoHyperparametersNEpochs::TextVariant(_)
    ));
    assert!(matches!(
        round_trips::<models::FineTuneReinforcementHyperparametersEvalInterval>("7"),
        models::FineTuneReinforcementHyperparametersEvalInterval::Integer(7)
    ));
    assert!(matches!(
        round_trips::<models::FineTuneReinforcementHyperparametersEvalInterval>("\"auto\""),
        models::FineTuneReinforcementHyperparametersEvalInterval::TextVariant(_)
    ));
    assert!(matches!(
        round_trips::<models::FineTuneReinforcementHyperparametersEvalSamples>("7"),
        models::FineTuneReinforcementHyperparametersEvalSamples::Integer(7)
    ));
    assert!(matches!(
        round_trips::<models::FineTuneReinforcementHyperparametersEvalSamples>("\"auto\""),
        models::FineTuneReinforcementHyperparametersEvalSamples::TextVariant(_)
    ));
    assert!(matches!(
        round_trips::<models::RealtimeBetaResponseCreateParamsMaxOutputTokens>("7"),
        models::RealtimeBetaResponseCreateParamsMaxOutputTokens::Integer(7)
    ));
    assert!(matches!(
        round_trips::<models::RealtimeBetaResponseCreateParamsMaxOutputTokens>("\"inf\""),
        models::RealtimeBetaResponseCreateParamsMaxOutputTokens::TextVariant(_)
    ));
    round_trips::<models::RealtimeBetaResponse>(r#####"{"max_output_tokens": 7}"#####);
    round_trips::<models::RealtimeBetaResponse>(r#####"{"max_output_tokens": "inf"}"#####);
}

#[test]
fn fields_holding_these_types_round_trip() {
    round_trips::<models::AuditLog>(
        r#####"{"effective_at": 1, "id": "x", "type": "api_key.created", "project": {"id": "x", "name": "x"}, "actor": {"type": "session", "session": {"user": {"id": "x", "email": "x"}, "ip_address": "x"}, "api_key": {"id": "x", "type": "user", "user": {"id": "x", "email": "x"}, "service_account": {"id": "x"}}}, "api_key.created": {"id": "x", "data": {"scopes": ["x"]}}, "api_key.updated": {"id": "x", "changes_requested": {"scopes": ["x"]}}, "api_key.deleted": {"id": "x"}, "checkpoint.permission.created": {"id": "x", "data": {"project_id": "x", "fine_tuned_model_checkpoint": "x"}}, "checkpoint.permission.deleted": {"id": "x"}, "external_key.registered": {"id": "x", "data": {"key": "value"}}, "external_key.removed": {"id": "x"}, "external_storage.registered": {"id": "x", "data": {"provider": {"account_id": "x", "bucket": "x", "external_id": "x", "region": "x", "role_arn": "x", "type": "aws"}, "geography": "x"}}, "external_storage.removed": {"id": "x"}, "group.created": {"id": "x", "data": {"group_name": "x"}}, "group.updated": {"id": "x", "changes_requested": {"group_name": "x"}}, "group.deleted": {"id": "x"}, "scim.enabled": {"id": "x"}, "scim.disabled": {"id": "x"}, "invite.sent": {"id": "x", "data": {"email": "x", "role": "x"}}, "invite.accepted": {"id": "x"}, "invite.deleted": {"id": "x"}, "ip_allowlist.created": {"id": "x", "name": "x", "allowed_ips": ["x"]}, "ip_allowlist.updated": {"id": "x", "allowed_ips": ["x"]}, "ip_allowlist.deleted": {"id": "x", "name": "x", "allowed_ips": ["x"]}, "ip_allowlist.config.activated": {"configs": [{"id": "x", "name": "x"}]}, "ip_allowlist.config.deactivated": {"configs": [{"id": "x", "name": "x"}]}, "login.succeeded": {"key": "value"}, "login.failed": {"error_code": "x", "error_message": "x"}, "logout.succeeded": {"key": "value"}, "logout.failed": {"error_code": "x", "error_message": "x"}, "organization.updated": {"id": "x", "changes_requested": {"title": "x", "description": "x", "name": "x", "threads_ui_visibility": "x", "usage_dashboard_visibility": "x", "api_call_logging": "x", "api_call_logging_project_ids": "x"}}, "project.created": {"id": "x", "data": {"name": "x", "title": "x"}}, "project.updated": {"id": "x", "changes_requested": {"title": "x"}}, "project.archived": {"id": "x"}, "project.deleted": {"id": "x"}, "rate_limit.updated": {"id": "x", "changes_requested": {"max_requests_per_1_minute": 1, "max_tokens_per_1_minute": 1, "max_images_per_1_minute": 1, "max_audio_megabytes_per_1_minute": 1, "max_requests_per_1_day": 1, "batch_1_day_max_input_tokens": 1}}, "rate_limit.deleted": {"id": "x"}, "role.created": {"id": "x", "role_name": "x", "permissions": ["x"], "resource_type": "x", "resource_id": "x"}, "role.updated": {"id": "x", "changes_requested": {"role_name": "x", "resource_id": "x", "resource_type": "x", "permissions_added": ["x"], "permissions_removed": ["x"], "description": "x", "metadata": {"key": "value"}}}, "role.deleted": {"id": "x"}, "role.assignment.created": {"id": "x", "principal_id": "x", "principal_type": "x", "resource_id": "x", "resource_type": "x"}, "role.assignment.deleted": {"id": "x", "principal_id": "x", "principal_type": "x", "resource_id": "x", "resource_type": "x"}, "role.bound_to_resource": {"id": "x", "role_id": "x", "resource_id": "x", "resource_type": "x", "permissions": ["x"], "workspace_id": "x", "connector_id": "x", "connector_name": "x", "enabled": true, "source": "role_toggle"}, "role.unbound_from_resource": {"id": "x", "role_id": "x", "resource_id": "x", "resource_type": "x", "permissions": ["x"], "workspace_id": "x", "connector_id": "x", "connector_name": "x", "enabled": true, "source": "role_toggle"}, "service_account.created": {"id": "x", "data": {"role": "x"}}, "service_account.updated": {"id": "x", "changes_requested": {"role": "x"}}, "service_account.deleted": {"id": "x"}, "workload_identity_provider.created": {"id": "x", "data": {"key": "value"}}, "workload_identity_provider.updated": {"id": "x", "changes_requested": {"key": "value"}}, "workload_identity_provider.deleted": {"id": "x", "name": "x"}, "workload_identity_provider_mapping.created": {"id": "x", "identity_provider_id": "x", "data": {"key": "value"}}, "workload_identity_provider_mapping.updated": {"id": "x", "identity_provider_id": "x", "changes_requested": {"key": "value"}}, "workload_identity_provider_mapping.deleted": {"id": "x", "identity_provider_id": "x", "project_id": "x", "service_account_id": "x"}, "user.added": {"id": "x", "data": {"role": "x"}}, "user.updated": {"id": "x", "changes_requested": {"role": "x"}}, "user.deleted": {"id": "x"}, "certificate.created": {"id": "x", "name": "x"}, "certificate.updated": {"id": "x", "name": "x"}, "certificate.deleted": {"id": "x", "name": "x", "certificate": "x"}, "certificates.activated": {"certificates": [{"id": "x", "name": "x"}]}, "certificates.deactivated": {"certificates": [{"id": "x", "name": "x"}]}}"#####,
    );
    round_trips::<models::McpToolCall>(
        r#####"{"arguments": "x", "id": "x", "name": "x", "server_label": "x", "type": "mcp_call", "output": "x", "error": {"code": 1, "message": "x", "type": "mcp_protocol_error"}, "status": "in_progress", "approval_request_id": "x"}"#####,
    );
    round_trips::<models::BetaMcpToolCall>(
        r#####"{"arguments": "x", "id": "x", "name": "x", "server_label": "x", "type": "mcp_call", "agent": {"agent_name": "x"}, "output": "x", "error": {"code": 1, "message": "x", "type": "mcp_protocol_error"}, "status": "in_progress", "approval_request_id": "x"}"#####,
    );
    round_trips::<models::FineTuningJob>(
        r#####"{"created_at": 1, "error": {"code": "x", "message": "x", "param": "x"}, "fine_tuned_model": "x", "finished_at": 1, "id": "x", "model": "x", "object": "fine_tuning.job", "organization_id": "x", "result_files": ["x"], "seed": 1, "status": "validating_files", "trained_tokens": 1, "training_file": "x", "validation_file": "x"}"#####,
    );
    round_trips::<models::RealtimeSession>(
        r#####"{"id": "x", "object": "realtime.session", "modalities": ["text"], "model": "x", "instructions": "x", "voice": "x", "input_audio_format": "pcm16", "output_audio_format": "pcm16", "input_audio_transcription": {"model": "x", "language": "x", "languages": ["x"], "prompt": "x"}, "turn_detection": {"type": "server_vad", "threshold": 0.5, "prefix_padding_ms": 1, "silence_duration_ms": 1, "create_response": true, "interrupt_response": true, "idle_timeout_ms": 5000}, "input_audio_noise_reduction": {"type": "near_field"}, "speed": 0.5, "tracing": "auto", "tools": [{"type": "function", "name": "x", "description": "x", "parameters": {"key": "value"}}], "tool_choice": "x", "temperature": 0.5, "max_response_output_tokens": 1, "expires_at": 1, "prompt": {"id": "x", "version": "x", "variables": {}}, "include": ["item.input_audio_transcription.logprobs"]}"#####,
    );
    round_trips::<models::RealtimeSessionCreateRequest>(
        r#####"{"client_secret": {"expires_at": 1, "value": "x"}, "modalities": ["text"], "instructions": "x", "voice": "x", "input_audio_format": "x", "output_audio_format": "x", "input_audio_transcription": {"model": "x"}, "speed": 0.5, "tracing": "auto", "turn_detection": {"type": "x", "threshold": 0.5, "prefix_padding_ms": 1, "silence_duration_ms": 1}, "tools": [{"type": "function", "name": "x", "description": "x", "parameters": {"key": "value"}}], "tool_choice": "x", "temperature": 0.5, "max_response_output_tokens": 1, "truncation": "auto", "prompt": {"id": "x", "version": "x", "variables": {}}}"#####,
    );
    round_trips::<models::RunObject>(
        r#####"{"assistant_id": "x", "cancelled_at": 1, "completed_at": 1, "created_at": 1, "expires_at": 1, "failed_at": 1, "id": "x", "incomplete_details": {}, "instructions": "x", "last_error": {"code": "server_error", "message": "x"}, "max_completion_tokens": 256, "max_prompt_tokens": 256, "metadata": {}, "model": "x", "object": "thread.run", "parallel_tool_calls": true, "required_action": {"submit_tool_outputs": {"tool_calls": [{"function": {"arguments": "x", "name": "x"}, "id": "x", "type": "function"}]}, "type": "submit_tool_outputs"}, "response_format": "auto", "started_at": 1, "status": "queued", "thread_id": "x", "tool_choice": "none", "tools": [{"type": "code_interpreter"}], "truncation_strategy": {"type": "auto"}, "usage": {"completion_tokens": 1, "prompt_tokens": 1, "total_tokens": 1}}"#####,
    );
    round_trips::<models::RunStepObject>(
        r#####"{"assistant_id": "x", "cancelled_at": 1, "completed_at": 1, "created_at": 1, "expired_at": 1, "failed_at": 1, "id": "x", "last_error": {"code": "server_error", "message": "x"}, "metadata": {}, "object": "thread.run.step", "run_id": "x", "status": "in_progress", "step_details": {"message_creation": {"message_id": "x"}, "type": "message_creation"}, "thread_id": "x", "type": "message_creation", "usage": {"completion_tokens": 1, "prompt_tokens": 1, "total_tokens": 1}}"#####,
    );
    let log = round_trips::<models::AuditLog>(
        r#####"{"effective_at": 1, "id": "x", "type": "api_key.created", "actor": {"type": "session", "session": {"ip_address": "127.0.0.1"}}}"#####,
    );
    assert_eq!(
        log.actor.flatten().unwrap().r#type,
        Some(models::audit_log_actor::Type::Session)
    );
}

/// Sends one request to a loopback server, and returns the multipart body it received.
async fn body_of<F, Fut>(call: F) -> String
where
    F: FnOnce(Configuration) -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut bytes = Vec::new();
        let mut chunk = [0u8; 4096];
        loop {
            let n = stream.read(&mut chunk).unwrap();
            bytes.extend_from_slice(&chunk[..n]);
            let text = String::from_utf8_lossy(&bytes);
            if let Some(at) = text.find("\r\n\r\n") {
                let length: usize = text[..at]
                    .lines()
                    .find_map(|l| {
                        l.to_ascii_lowercase()
                            .strip_prefix("content-length: ")
                            .map(|v| v.trim().parse().unwrap())
                    })
                    .unwrap();
                if bytes.len() >= at + 4 + length {
                    break;
                }
            }
        }
        stream
            .write_all(b"HTTP/1.1 500 Internal Server Error\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}")
            .unwrap();
        String::from_utf8_lossy(&bytes).into_owned()
    });
    let configuration = Configuration {
        base_path: format!("http://{address}"),
        ..Configuration::default()
    };
    call(configuration).await;
    server.join().unwrap()
}

fn part<'a>(body: &'a str, name: &str) -> &'a str {
    let header = format!("name=\"{name}\"\r\n\r\n");
    let start = body
        .find(&header)
        .unwrap_or_else(|| panic!("no part {name} in {body}"))
        + header.len();
    let end = body[start..].find("\r\n").unwrap() + start;
    &body[start..end]
}

#[tokio::test]
#[allow(deprecated)]
async fn a_string_variant_goes_out_as_plain_text() {
    let body = body_of(|configuration| async move {
        let _ = videos_api::create_video()
            .configuration(&configuration)
            .prompt("a cat")
            .model(models::VideoModel::from("sora-2"))
            .call()
            .await;
    })
    .await;
    assert_eq!(part(&body, "model"), "sora-2");

    let body = body_of(|configuration| async move {
        let _ = videos_api::create_video()
            .configuration(&configuration)
            .prompt("a cat")
            .model(serde_json::from_str::<models::VideoModel>("\"sora-2-pro\"").unwrap())
            .call()
            .await;
    })
    .await;
    assert_eq!(part(&body, "model"), "sora-2-pro");
}

#[tokio::test]
async fn image_edit_size_goes_out_as_plain_text() {
    let image = std::env::temp_dir().join(format!("model-unions-{}.png", std::process::id()));
    std::fs::write(&image, b"png").unwrap();
    let path = image.clone();
    let body = body_of(|configuration| async move {
        let _ = images_api::create_image_edit()
            .configuration(&configuration)
            .image(path)
            .prompt("a cat")
            .model("gpt-image-1")
            .size(models::CreateImageEditRequestSize::from("1024x1024"))
            .call()
            .await;
    })
    .await;
    std::fs::remove_file(&image).ok();
    assert_eq!(part(&body, "size"), "1024x1024");
}

#[test]
fn helpers_keep_their_names() {
    let text = models::ChatCompletionRequestUserMessageContent::new_text("hi".to_string());
    assert_eq!(
        text,
        models::ChatCompletionRequestUserMessageContent::from("hi")
    );
    assert_eq!(
        text,
        models::ChatCompletionRequestUserMessageContent::from("hi".to_string())
    );
    assert_eq!(text.to_string(), "hi");
}
