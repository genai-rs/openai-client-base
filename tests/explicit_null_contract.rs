use openai_client_base::models::{
    ChatCompletionStreamResponseDelta, CreateChatCompletionRequest,
    CreateChatCompletionStreamResponseChoicesInner, CreateCompletionRequest,
    CreateFineTuningJobRequest, CreateFineTuningJobRequestIntegrationsInnerWandb,
    CreateImageRequest, CreateRunRequest, CreateThreadAndRunRequest, LiveHostedShellToolInputParam,
    LiveMcpToolChoiceParam, PredictionContent, PromptCacheOptions, RealtimeSessionCreateRequestGa,
    StopConfiguration, UpdateVectorStoreRequest,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};

fn check_field<T>(base: Value, field: &str, value: Value)
where
    T: DeserializeOwned + Serialize,
{
    let mut omitted = base.clone();
    omitted.as_object_mut().unwrap().remove(field);
    let parsed: T = serde_json::from_value(omitted.clone()).unwrap();
    let encoded = serde_json::to_value(parsed).unwrap();
    assert!(encoded.get(field).is_none(), "{field} should stay omitted");

    let mut explicit_null = base.clone();
    explicit_null[field] = Value::Null;
    let parsed: T = serde_json::from_value(explicit_null).unwrap();
    let encoded = serde_json::to_value(parsed).unwrap();
    assert_eq!(encoded.get(field), Some(&Value::Null), "{field}");

    let mut present = base;
    present[field] = value.clone();
    let parsed: T = serde_json::from_value(present).unwrap();
    let encoded = serde_json::to_value(parsed).unwrap();
    assert_eq!(encoded.get(field), Some(&value), "{field}");
}

fn check_fields<T>(base: Value, fields: &[(&str, Value)])
where
    T: DeserializeOwned + Serialize,
{
    for (field, value) in fields {
        check_field::<T>(base.clone(), field, value.clone());
    }
}

fn assert_required_field<T>(base: Value, field: &str)
where
    T: DeserializeOwned,
{
    let mut missing = base;
    missing.as_object_mut().unwrap().remove(field);
    assert!(
        serde_json::from_value::<T>(missing).is_err(),
        "missing required field {field} should be rejected"
    );
}

fn assert_omitted_fields<T>(model: T, fields: &[&str])
where
    T: Serialize,
{
    let encoded = serde_json::to_value(model).unwrap();
    for field in fields {
        assert!(
            encoded.get(field).is_none(),
            "constructor should omit {field} by default"
        );
    }
}

#[test]
fn affected_request_fields_preserve_omitted_null_and_value() {
    check_fields::<CreateChatCompletionRequest>(
        json!({"messages": [], "model": "gpt-4o"}),
        &[
            ("top_logprobs", json!(0)),
            ("max_completion_tokens", json!(0)),
            ("frequency_penalty", json!(0.0)),
            ("presence_penalty", json!(0.0)),
            ("store", json!(false)),
            ("stream", json!(false)),
            ("stop", json!("")),
            ("logprobs", json!(false)),
            ("max_tokens", json!(0)),
            ("n", json!(0)),
            ("prediction", json!({"type": "content", "content": ""})),
            ("seed", json!(0)),
        ],
    );
    check_fields::<CreateCompletionRequest>(
        json!({"model": "text-davinci-003", "prompt": ""}),
        &[
            ("best_of", json!(0)),
            ("echo", json!(false)),
            ("frequency_penalty", json!(0.0)),
            ("logprobs", json!(0)),
            ("max_tokens", json!(0)),
            ("n", json!(0)),
            ("presence_penalty", json!(0.0)),
            ("seed", json!(0)),
            ("stop", json!("")),
            ("stream", json!(false)),
            ("suffix", json!("")),
            ("temperature", json!(0.0)),
            ("top_p", json!(0.0)),
        ],
    );
    check_fields::<CreateImageRequest>(
        json!({"prompt": "", "model": "gpt-image-1"}),
        &[
            ("n", json!(0)),
            ("quality", json!("low")),
            ("response_format", json!("url")),
            ("output_format", json!("png")),
            ("output_compression", json!(0)),
            ("stream", json!(false)),
            ("size", json!("")),
            ("moderation", json!("low")),
            ("background", json!("auto")),
            ("style", json!("natural")),
        ],
    );
    check_fields::<CreateRunRequest>(
        json!({"assistant_id": "asst_test"}),
        &[
            ("instructions", json!("")),
            ("additional_instructions", json!("")),
            ("additional_messages", json!([])),
            ("tools", json!([])),
            ("temperature", json!(0.0)),
            ("top_p", json!(0.0)),
            ("stream", json!(false)),
            ("max_prompt_tokens", json!(0)),
            ("max_completion_tokens", json!(0)),
        ],
    );
    check_fields::<CreateThreadAndRunRequest>(
        json!({"assistant_id": "asst_test"}),
        &[
            ("instructions", json!("")),
            ("tools", json!([])),
            ("temperature", json!(0.0)),
            ("top_p", json!(0.0)),
            ("stream", json!(false)),
            ("max_prompt_tokens", json!(0)),
            ("max_completion_tokens", json!(0)),
        ],
    );
    check_fields::<UpdateVectorStoreRequest>(json!({}), &[("name", json!(""))]);
    check_fields::<CreateFineTuningJobRequest>(
        json!({"model": "babbage-002", "training_file": "file_train"}),
        &[
            ("suffix", json!("")),
            ("validation_file", json!("")),
            ("integrations", json!([])),
            ("seed", json!(0)),
        ],
    );
    check_fields::<CreateFineTuningJobRequestIntegrationsInnerWandb>(
        json!({"project": ""}),
        &[("name", json!("")), ("entity", json!(""))],
    );
    check_fields::<RealtimeSessionCreateRequestGa>(
        json!({"type": "realtime"}),
        &[("tracing", json!("auto"))],
    );
    check_fields::<LiveHostedShellToolInputParam>(
        json!({"type": "shell"}),
        &[("environment", json!({"type": "container_auto"}))],
    );
    check_fields::<LiveMcpToolChoiceParam>(
        json!({"type": "mcp", "server_label": "server"}),
        &[("name", json!(""))],
    );
    check_fields::<PromptCacheOptions>(json!({"ttl": "30m"}), &[("mode", json!("implicit"))]);
    check_fields::<CreateChatCompletionStreamResponseChoicesInner>(
        json!({"delta": {}, "index": 0}),
        &[("finish_reason", json!("stop"))],
    );
}

#[test]
fn constructors_keep_their_required_arguments_and_usable_defaults() {
    assert_omitted_fields(
        CreateChatCompletionRequest::new(vec![], "gpt-4o".to_owned()),
        &[
            "top_logprobs",
            "max_completion_tokens",
            "frequency_penalty",
            "presence_penalty",
            "store",
            "stream",
            "stop",
            "logprobs",
            "max_tokens",
            "n",
            "prediction",
            "seed",
        ],
    );
    assert_omitted_fields(
        CreateCompletionRequest::new("text-davinci-003".to_owned(), "".to_owned()),
        &[
            "best_of",
            "echo",
            "frequency_penalty",
            "logprobs",
            "max_tokens",
            "n",
            "presence_penalty",
            "seed",
            "stop",
            "stream",
            "suffix",
            "temperature",
            "top_p",
        ],
    );
    assert_omitted_fields(
        CreateImageRequest::new("".to_owned(), "gpt-image-1".to_owned()),
        &[
            "n",
            "quality",
            "response_format",
            "output_format",
            "output_compression",
            "stream",
            "size",
            "moderation",
            "background",
            "style",
        ],
    );
    assert_omitted_fields(
        CreateImageRequest::default(),
        &[
            "n",
            "quality",
            "response_format",
            "output_format",
            "output_compression",
            "stream",
            "size",
            "moderation",
            "background",
            "style",
        ],
    );
    assert_omitted_fields(
        CreateRunRequest::new("asst_test".to_owned()),
        &[
            "instructions",
            "additional_instructions",
            "additional_messages",
            "tools",
            "temperature",
            "top_p",
            "stream",
            "max_prompt_tokens",
            "max_completion_tokens",
        ],
    );
    assert_omitted_fields(
        CreateThreadAndRunRequest::new("asst_test".to_owned()),
        &[
            "instructions",
            "tools",
            "temperature",
            "top_p",
            "stream",
            "max_prompt_tokens",
            "max_completion_tokens",
        ],
    );
    assert_omitted_fields(UpdateVectorStoreRequest::new(), &["name"]);
    assert_omitted_fields(
        CreateFineTuningJobRequest::new("babbage-002".to_owned(), "file_train".to_owned()),
        &["suffix", "validation_file", "integrations", "seed"],
    );
    assert_omitted_fields(
        CreateFineTuningJobRequestIntegrationsInnerWandb::new("project".to_owned()),
        &["name", "entity"],
    );
    assert_omitted_fields(
        RealtimeSessionCreateRequestGa::new(
            openai_client_base::models::realtime_session_create_request_ga::Type::Realtime,
        ),
        &["tracing"],
    );
    assert_omitted_fields(
        LiveHostedShellToolInputParam::new(
            openai_client_base::models::live_hosted_shell_tool_input_param::Type::Shell,
        ),
        &["environment"],
    );
    assert_omitted_fields(
        LiveMcpToolChoiceParam::new(
            openai_client_base::models::live_mcp_tool_choice_param::Type::Mcp,
            "server".to_owned(),
        ),
        &["name"],
    );
    assert_omitted_fields(PromptCacheOptions::new(), &["ttl", "mode"]);
    assert_omitted_fields(
        CreateChatCompletionStreamResponseChoicesInner::new(
            ChatCompletionStreamResponseDelta::new(),
            0,
        ),
        &["finish_reason"],
    );

    let prediction = PredictionContent::new(
        openai_client_base::models::prediction_content::Type::Content,
        json!(""),
    );
    assert_eq!(
        serde_json::to_value(prediction).unwrap(),
        json!({"type": "content", "content": ""})
    );
    assert_eq!(
        serde_json::to_value(StopConfiguration::from("")).unwrap(),
        json!("")
    );
}

#[test]
fn finish_reason_variants_and_missing_state_are_explicit() {
    use openai_client_base::models::create_chat_completion_stream_response_choices_inner::FinishReason;

    let variants = [
        (FinishReason::Stop, "stop"),
        (FinishReason::Length, "length"),
        (FinishReason::ToolCalls, "tool_calls"),
        (FinishReason::ContentFilter, "content_filter"),
        (FinishReason::FunctionCall, "function_call"),
    ];
    for (variant, wire) in variants {
        assert_eq!(serde_json::to_value(variant).unwrap(), json!(wire));
        let parsed: FinishReason = serde_json::from_value(json!(wire)).unwrap();
        assert_eq!(parsed, variant);
        let choice: CreateChatCompletionStreamResponseChoicesInner =
            serde_json::from_value(json!({"delta": {}, "index": 0, "finish_reason": wire}))
                .unwrap();
        assert_eq!(choice.finish_reason, Some(Some(variant)));
        assert_eq!(
            serde_json::to_value(choice).unwrap().get("finish_reason"),
            Some(&json!(wire))
        );
    }

    let missing: CreateChatCompletionStreamResponseChoicesInner =
        serde_json::from_value(json!({"delta": {}, "index": 0})).unwrap();
    assert_eq!(missing.finish_reason, None);
    let null: CreateChatCompletionStreamResponseChoicesInner =
        serde_json::from_value(json!({"delta": {}, "index": 0, "finish_reason": null})).unwrap();
    assert_eq!(null.finish_reason, Some(None));
    let value: CreateChatCompletionStreamResponseChoicesInner = serde_json::from_value(json!({
        "delta": {}, "index": 0, "finish_reason": "tool_calls"
    }))
    .unwrap();
    assert_eq!(value.finish_reason, Some(Some(FinishReason::ToolCalls)));
}

#[test]
fn affected_models_reject_missing_required_fields() {
    let chat = json!({"messages": [], "model": "gpt-4o"});
    assert_required_field::<CreateChatCompletionRequest>(chat.clone(), "messages");
    assert_required_field::<CreateChatCompletionRequest>(chat, "model");

    let completion = json!({"model": "text-davinci-003", "prompt": ""});
    assert_required_field::<CreateCompletionRequest>(completion.clone(), "model");
    assert_required_field::<CreateCompletionRequest>(completion, "prompt");

    let image = json!({"prompt": "", "model": "gpt-image-1"});
    assert_required_field::<CreateImageRequest>(image.clone(), "prompt");
    assert_required_field::<CreateImageRequest>(image, "model");

    assert_required_field::<CreateRunRequest>(json!({"assistant_id": "asst_test"}), "assistant_id");
    assert_required_field::<CreateThreadAndRunRequest>(
        json!({"assistant_id": "asst_test"}),
        "assistant_id",
    );

    let fine_tune = json!({"model": "babbage-002", "training_file": "file_train"});
    assert_required_field::<CreateFineTuningJobRequest>(fine_tune.clone(), "model");
    assert_required_field::<CreateFineTuningJobRequest>(fine_tune, "training_file");
    assert_required_field::<CreateFineTuningJobRequestIntegrationsInnerWandb>(
        json!({"project": ""}),
        "project",
    );
    assert_required_field::<RealtimeSessionCreateRequestGa>(json!({"type": "realtime"}), "type");
    assert_required_field::<LiveHostedShellToolInputParam>(json!({"type": "shell"}), "type");
    assert_required_field::<LiveMcpToolChoiceParam>(
        json!({"type": "mcp", "server_label": "server"}),
        "server_label",
    );
    assert_required_field::<LiveMcpToolChoiceParam>(
        json!({"type": "mcp", "server_label": "server"}),
        "type",
    );
    // The generated model accepts omission despite both compared schema revisions
    // requiring these fields.
    let missing_prompt_cache_fields: PromptCacheOptions =
        serde_json::from_value(json!({})).unwrap();
    assert_eq!(missing_prompt_cache_fields.ttl, None);
    assert_eq!(missing_prompt_cache_fields.mode, None);
    let explicit_nulls: PromptCacheOptions =
        serde_json::from_value(json!({"ttl": null, "mode": null})).unwrap();
    assert_eq!(explicit_nulls.ttl, None);
    assert_eq!(explicit_nulls.mode, Some(None));
    let encoded = serde_json::to_value(explicit_nulls).unwrap();
    assert!(encoded.get("ttl").is_none());
    assert_eq!(encoded.get("mode"), Some(&Value::Null));

    let choice = json!({"delta": {}, "index": 0});
    assert_required_field::<CreateChatCompletionStreamResponseChoicesInner>(
        choice.clone(),
        "delta",
    );
    assert_required_field::<CreateChatCompletionStreamResponseChoicesInner>(choice, "index");
}
