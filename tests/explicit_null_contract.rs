use openai_client_base::models::{
    ChatCompletionStreamResponseDelta, CreateChatCompletionRequest,
    CreateChatCompletionStreamResponseChoicesInner, CreateCompletionRequest,
    CreateFineTuningJobRequest, CreateFineTuningJobRequestIntegrationsInnerWandb,
    CreateImageRequest, CreateRunRequest, CreateThreadAndRunRequest, PredictionContent,
    RealtimeSessionCreateRequestGa, StopConfiguration, UpdateVectorStoreRequest,
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
    check_fields::<CreateChatCompletionStreamResponseChoicesInner>(
        json!({"delta": {}, "index": 0}),
        &[("finish_reason", json!("stop"))],
    );
}

#[test]
fn constructors_keep_their_required_arguments_and_usable_defaults() {
    let _ = CreateChatCompletionRequest::new(vec![], "gpt-4o".to_owned());
    let _ = CreateCompletionRequest::new("text-davinci-003".to_owned(), "".to_owned());
    let _ = CreateImageRequest::new("".to_owned(), "gpt-image-1".to_owned());
    let _ = CreateRunRequest::new("asst_test".to_owned());
    let _ = CreateThreadAndRunRequest::new("asst_test".to_owned());
    let _ = UpdateVectorStoreRequest::new();
    let _ = CreateFineTuningJobRequest::new("babbage-002".to_owned(), "file_train".to_owned());
    let _ = CreateChatCompletionStreamResponseChoicesInner::new(
        ChatCompletionStreamResponseDelta::new(),
        0,
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
