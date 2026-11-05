//! What `openEnums` does on the wire. A value the spec does not list must not fail a response, and everything that relies
//! on an enum rejecting a value (`FromStr`, requests, decoding in order) must keep rejecting it.

use std::str::FromStr;

use openai_client_base::apis::{chat_api, configuration::Configuration};
use openai_client_base::models;
use openai_client_base::models::create_chat_completion_response_choices_inner::FinishReason;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// A chat completion with a value the spec does not list at every depth, around a message that is fine.
fn completion_with_new_values() -> Value {
    json!({
        "id": "chatcmpl-1",
        "object": "chat.completion.v2",
        "created": 1_700_000_000,
        "model": "gpt-test",
        "service_tier": "brand_new_tier",
        "choices": [{
            "index": 0,
            "finish_reason": "brand_new_reason",
            "logprobs": null,
            "message": {
                "role": "brand_new_role",
                "content": "the rest of the response is still here",
                "refusal": null,
                "annotations": [{
                    "type": "brand_new_annotation",
                    "url_citation": {"start_index": 1, "end_index": 2, "url": "https://example.com", "title": "t"}
                }],
                "tool_calls": [{"type": "brand_new_tool", "id": "call_1", "surprise": {"a": [1, 2]}}]
            }
        }]
    })
}

fn assert_new_values_decoded(response: &models::CreateChatCompletionResponse) {
    let choice = &response.choices[0];
    assert_eq!(choice.finish_reason, FinishReason::Unknown);
    assert_eq!(
        response.service_tier,
        Some(Some(models::ServiceTier::Unknown))
    );
    assert_eq!(
        response.object,
        models::create_chat_completion_response::Object::Unknown
    );
    assert_eq!(
        choice.message.role,
        models::chat_completion_response_message::Role::Unknown
    );
    assert_eq!(
        choice.message.content.as_deref(),
        Some("the rest of the response is still here")
    );
    assert_eq!(
        choice.message.annotations.as_ref().unwrap()[0].r#type,
        models::chat_completion_response_message_annotations_inner::Type::Unknown
    );
    assert_eq!(
        choice
            .message
            .tool_calls
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap()[0],
        models::ChatCompletionMessageToolCallsInner::Unknown(
            json!({"type": "brand_new_tool", "id": "call_1", "surprise": {"a": [1, 2]}})
        )
    );
}

#[test]
fn a_new_value_deep_inside_a_response_does_not_fail_it() {
    let response: models::CreateChatCompletionResponse =
        serde_json::from_value(completion_with_new_values()).unwrap();
    assert_new_values_decoded(&response);
}

#[tokio::test]
async fn a_new_value_does_not_fail_an_api_call() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/chat/completions/chatcmpl-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(completion_with_new_values()))
        .expect(1)
        .mount(&server)
        .await;
    let configuration = Configuration {
        base_path: server.uri(),
        ..Configuration::default()
    };

    let response = chat_api::get_chat_completion()
        .configuration(&configuration)
        .completion_id("chatcmpl-1")
        .call()
        .await
        .expect("a value the spec does not list should not fail the response");

    assert_new_values_decoded(&response);
}

#[test]
fn one_new_value_in_a_list_keeps_the_other_items() {
    let batch = |id: &str, status: &str| {
        json!({
            "id": id,
            "object": "batch",
            "endpoint": "/v1/responses",
            "input_file_id": "file-1",
            "completion_window": "24h",
            "status": status,
            "created_at": 1_700_000_000
        })
    };
    let list: models::ListBatchesResponse = serde_json::from_value(json!({
        "object": "list",
        "has_more": false,
        "data": [batch("batch_1", "completed"), batch("batch_2", "brand_new_status")]
    }))
    .unwrap();

    assert_eq!(list.data[0].status, models::batch::Status::Completed);
    assert_eq!(list.data[1].status, models::batch::Status::Unknown);
    assert_eq!(list.data[1].id, "batch_2");
}

#[test]
fn unknown_of_an_enum_does_not_keep_the_value() {
    let reason: FinishReason = serde_json::from_str(r#""brand_new_reason""#).unwrap();
    assert_eq!(reason, FinishReason::Unknown);

    // The string is gone: this is what goes out, and it decodes to `Unknown` again.
    assert_eq!(serde_json::to_string(&reason).unwrap(), r#""unknown""#);
    assert_eq!(reason.to_string(), "unknown");
    assert_eq!(
        serde_json::from_str::<FinishReason>(r#""unknown""#).unwrap(),
        FinishReason::Unknown
    );

    // Listed values are untouched.
    assert_eq!(
        serde_json::from_str::<FinishReason>(r#""tool_calls""#).unwrap(),
        FinishReason::ToolCalls
    );
    assert_eq!(FinishReason::ToolCalls.to_string(), "tool_calls");

    // Another JSON kind is still an error.
    assert!(serde_json::from_str::<FinishReason>("3").is_err());
    assert!(serde_json::from_str::<FinishReason>("null").is_err());

    // serde reads an object with one key as an enum, as for any externally tagged enum.
    assert_eq!(
        serde_json::from_str::<FinishReason>(r#"{"brand_new_reason": null}"#).unwrap(),
        FinishReason::Unknown
    );
}

#[test]
fn an_integer_enum_has_a_catch_all_too() {
    use models::pcm_audio_format::Rate;

    assert_eq!(
        serde_json::from_str::<Rate>("24000").unwrap(),
        Rate::Variant24000
    );
    let rate: Rate = serde_json::from_str("48000").unwrap();
    assert_eq!(rate, Rate::Unknown);
    assert_eq!(serde_json::to_value(rate).unwrap(), json!(i64::MAX));
    assert_eq!(rate.to_string(), "unknown");
    assert!("48000".parse::<Rate>().is_err());
}

#[test]
fn an_enum_that_lists_unknown_itself_uses_unknown_value() {
    use models::image_gen_tool_call_moderation_details::ModerationStage;
    use models::MessageRole;

    // `unknown` is a real value of these enums, so it stays what it was, on the wire and in `FromStr`.
    assert_eq!(
        serde_json::from_str::<MessageRole>(r#""unknown""#).unwrap(),
        MessageRole::Unknown
    );
    assert_eq!(
        "unknown".parse::<MessageRole>().unwrap(),
        MessageRole::Unknown
    );
    assert_eq!(
        serde_json::to_string(&MessageRole::Unknown).unwrap(),
        r#""unknown""#
    );

    // A new value goes to the catch-all, which has its own spelling.
    let role: MessageRole = serde_json::from_str(r#""brand_new_role""#).unwrap();
    assert_eq!(role, MessageRole::UnknownValue);
    assert_eq!(serde_json::to_string(&role).unwrap(), r#""unknown_value""#);
    assert_eq!(role.to_string(), "unknown_value");
    assert!("brand_new_role".parse::<MessageRole>().is_err());
    assert!("unknown_value".parse::<MessageRole>().is_err());

    assert_eq!(
        serde_json::from_str::<ModerationStage>(r#""unknown""#).unwrap(),
        ModerationStage::Unknown
    );
    assert_eq!(
        serde_json::from_str::<ModerationStage>(r#""brand_new_stage""#).unwrap(),
        ModerationStage::UnknownValue
    );
    assert!("unknown_value".parse::<ModerationStage>().is_err());
}

fn unknown_tag_round_trips<T>(sample: Value, is_unknown: impl Fn(&T) -> Option<&Value>)
where
    T: DeserializeOwned + Serialize + std::fmt::Display,
{
    let union: T = serde_json::from_value(sample.clone()).unwrap();
    assert_eq!(is_unknown(&union), Some(&sample));
    assert_eq!(serde_json::to_value(&union).unwrap(), sample);
    assert_eq!(union.to_string(), sample.to_string());
}

#[test]
fn an_unknown_tag_on_a_union_keeps_the_json() {
    use models::{Annotation, AssistantStreamEvent, ChatCompletionMessageToolCallsInner};

    unknown_tag_round_trips::<ChatCompletionMessageToolCallsInner>(
        json!({"type": "brand_new_tool", "id": "call_1", "nested": {"list": [1, null, "x"], "n": 1.5}}),
        |union| match union {
            ChatCompletionMessageToolCallsInner::Unknown(value) => Some(value),
            _ => None,
        },
    );
    unknown_tag_round_trips::<Annotation>(
        json!({"type": "brand_new_annotation", "start_index": 3}),
        |union| match union {
            Annotation::Unknown(value) => Some(value),
            _ => None,
        },
    );
    // The tag of this one is `event`.
    unknown_tag_round_trips::<AssistantStreamEvent>(
        json!({"event": "thread.brand_new", "data": {"id": "x"}}),
        |union| match union {
            AssistantStreamEvent::Unknown(value) => Some(value),
            _ => None,
        },
    );
}

#[test]
fn a_union_only_takes_a_tag_it_does_not_know() {
    use models::ChatCompletionMessageToolCallsInner as ToolCall;

    // No tag to read: still an error, as without the option.
    assert!(serde_json::from_value::<ToolCall>(json!({"id": "call_1"})).is_err());
    assert!(serde_json::from_value::<ToolCall>(json!({"type": null})).is_err());
    assert!(serde_json::from_value::<ToolCall>(json!({"type": 3})).is_err());
    assert!(serde_json::from_value::<ToolCall>(json!("function")).is_err());

    // A tag it knows with a broken body is that variant's error, not `Unknown`.
    let error = serde_json::from_value::<ToolCall>(json!({"type": "function", "id": "call_1"}))
        .unwrap_err()
        .to_string();
    assert!(error.contains("function"), "{error}");

    // A tag it knows still decodes to its variant.
    let call: ToolCall = serde_json::from_value(json!({
        "type": "function",
        "id": "call_1",
        "function": {"name": "f", "arguments": "{}"}
    }))
    .unwrap();
    assert!(matches!(call, ToolCall::ChatCompletionMessageToolCall(_)));
}

#[test]
fn from_str_never_produces_the_catch_all() {
    for text in ["brand_new_reason", "unknown", "Unknown", ""] {
        let error = FinishReason::from_str(text).unwrap_err();
        assert_eq!(error.value(), text);
        assert!(FinishReason::try_from(text).is_err(), "{text}");
        assert!(FinishReason::try_from(text.to_string()).is_err(), "{text}");
        assert!(FinishReason::try_from(&text.to_string()).is_err(), "{text}");
    }
    assert_eq!("stop".parse::<FinishReason>().unwrap(), FinishReason::Stop);

    // Decoding takes what `FromStr` refuses: it is the one that reads what the API sent.
    assert_eq!(
        serde_json::from_str::<FinishReason>(r#""unknown""#).unwrap(),
        FinishReason::Unknown
    );
}

/// Exhaustive on purpose: this stops compiling if the enum gains a catch-all.
fn request_only_format_label(format: models::AudioResponseFormat) -> &'static str {
    match format {
        models::AudioResponseFormat::Json => "json",
        models::AudioResponseFormat::Text => "text",
        models::AudioResponseFormat::Srt => "srt",
        models::AudioResponseFormat::VerboseJson => "verbose_json",
        models::AudioResponseFormat::Vtt => "vtt",
        models::AudioResponseFormat::DiarizedJson => "diarized_json",
    }
}

#[test]
fn what_only_a_request_uses_stays_closed() {
    let format: models::AudioResponseFormat = serde_json::from_str(r#""srt""#).unwrap();
    assert_eq!(request_only_format_label(format), "srt");
    assert!(serde_json::from_str::<models::AudioResponseFormat>(r#""brand_new""#).is_err());
    assert!(serde_json::from_str::<models::AudioResponseFormat>(r#""unknown""#).is_err());

    // A union only requests use keeps the error for an unknown tag.
    let error = serde_json::from_value::<models::ChatCompletionRequestMessage>(
        json!({"role": "brand_new", "content": "x"}),
    )
    .unwrap_err()
    .to_string();
    assert!(error.contains("brand_new"), "{error}");
}

#[test]
fn a_guarded_enum_in_front_of_a_free_string_leaves_it_the_string() {
    use models::{ImagesResponseSize, ImagesResponseSizeTextVariantEnum};

    // A value the enum does not list is what the string is for: a catch-all would lose it.
    let size: ImagesResponseSize = serde_json::from_str(r#""1536x864""#).unwrap();
    assert_eq!(size, ImagesResponseSize::Text("1536x864".to_string()));
    assert_eq!(serde_json::to_string(&size).unwrap(), r#""1536x864""#);

    // So is the spelling of a catch-all, since that enum does not have one.
    let size: ImagesResponseSize = serde_json::from_str(r#""unknown""#).unwrap();
    assert_eq!(size, ImagesResponseSize::Text("unknown".to_string()));

    let size: ImagesResponseSize = serde_json::from_str(r#""1024x1024""#).unwrap();
    assert!(matches!(size, ImagesResponseSize::TextVariant(_)));

    assert!(serde_json::from_str::<ImagesResponseSizeTextVariantEnum>(r#""1536x864""#).is_err());
}

#[test]
fn a_guarded_tagged_union_keeps_the_error() {
    let error = serde_json::from_value::<models::InputContent>(json!({"type": "brand_new"}))
        .unwrap_err()
        .to_string();
    assert!(error.contains("brand_new"), "{error}");
}

#[test]
fn a_new_value_in_a_guarded_enum_still_fails_when_nothing_takes_it() {
    let screenshot = |detail: &str| json!({"type": "computer_screenshot", "image_url": "https://example.com/a.png", "file_id": null, "detail": detail});
    assert!(serde_json::from_value::<models::ComputerScreenshotContent>(screenshot("low")).is_ok());
    let error =
        serde_json::from_value::<models::ComputerScreenshotContent>(screenshot("brand_new"))
            .unwrap_err()
            .to_string();
    assert!(error.contains("brand_new"), "{error}");
}

#[test]
fn a_role_that_tells_two_structs_apart_stays_closed() {
    assert!(serde_json::from_value::<models::InputMessage>(json!({
        "role": "brand_new",
        "content": []
    }))
    .is_err());

    // The other fields of that struct are free to open.
    let message: models::InputMessage = serde_json::from_value(json!({
        "role": "user",
        "status": "brand_new_status",
        "content": []
    }))
    .unwrap();
    assert_eq!(message.status, Some(models::input_message::Status::Unknown));
}
