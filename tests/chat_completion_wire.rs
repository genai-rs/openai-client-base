//! Chat completion bodies as the API sends them. `choices[].logprobs` is required and nullable in
//! the spec: `gpt-4o-mini` and `gpt-4.1-mini` send `"logprobs": null`, `gpt-5-mini` leaves it out.
//! A required nullable field that is missing decodes as `None` (lenient on input), and `None` is
//! written as `null` (strict on output).

use openai_client_base::models::create_chat_completion_response_choices_inner::FinishReason;
use openai_client_base::models::CreateChatCompletionResponse;
use serde_json::{json, Value};

fn completion(choice: Value) -> Value {
    json!({
        "id": "chatcmpl-123",
        "object": "chat.completion",
        "created": 1_763_500_000,
        "model": "gpt-5-mini-2025-08-07",
        "choices": [choice],
        "usage": {"prompt_tokens": 9, "completion_tokens": 12, "total_tokens": 21},
        "service_tier": "default",
        "system_fingerprint": null
    })
}

fn message() -> Value {
    json!({"role": "assistant", "content": "Hello!", "refusal": null, "annotations": []})
}

#[test]
fn a_choice_without_logprobs_decodes() {
    let body = completion(json!({"index": 0, "message": message(), "finish_reason": "stop"}));
    let response: CreateChatCompletionResponse = serde_json::from_value(body).unwrap();
    let choice = &response.choices[0];
    assert_eq!(choice.logprobs, None);
    assert_eq!(choice.finish_reason, FinishReason::Stop);
    assert_eq!(choice.message.content.as_deref(), Some("Hello!"));
    // written back with an explicit null, as the spec requires
    let encoded = serde_json::to_value(&response).unwrap();
    assert_eq!(encoded["choices"][0]["logprobs"], Value::Null);
}

#[test]
fn a_choice_with_null_logprobs_decodes() {
    let body = completion(json!({
        "index": 0, "message": message(), "finish_reason": "stop", "logprobs": null
    }));
    let response: CreateChatCompletionResponse = serde_json::from_value(body).unwrap();
    assert_eq!(response.choices[0].logprobs, None);
}

#[test]
fn a_choice_with_logprobs_decodes() {
    let body = completion(json!({
        "index": 0,
        "message": message(),
        "finish_reason": "stop",
        "logprobs": {
            "content": [{"token": "Hello", "logprob": -0.1, "bytes": [72, 101, 108, 108, 111], "top_logprobs": []}],
            "refusal": null
        }
    }));
    let response: CreateChatCompletionResponse = serde_json::from_value(body).unwrap();
    let logprobs = response.choices[0].logprobs.as_ref().expect("logprobs");
    assert_eq!(logprobs.content.as_ref().map(Vec::len), Some(1));
}

#[test]
fn a_choice_without_its_required_fields_still_fails() {
    let body = completion(json!({"index": 0, "finish_reason": "stop"}));
    let error = serde_json::from_value::<CreateChatCompletionResponse>(body).unwrap_err();
    assert!(error.to_string().contains("message"), "{error}");
}
