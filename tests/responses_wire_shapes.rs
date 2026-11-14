//! Shapes the Responses API sends and accepts that the spec types more strictly: `instructions`
//! is null without instructions, `logprobs` and `cache_write_tokens` are left out, and `input`
//! may be a list of items instead of a string.
#![cfg(feature = "responses-types")]

use openai_client_base::models::{CreateResponse, InputParam, Response};
use serde_json::json;

fn api_reference_response() -> serde_json::Value {
    json!({
        "id": "resp_67ccd2bed1ec8190b14f964abc0542670bb6a6b452d3795b",
        "object": "response",
        "created_at": 1741476542,
        "status": "completed",
        "error": null,
        "incomplete_details": null,
        "instructions": null,
        "max_output_tokens": null,
        "model": "gpt-4.1-2025-04-14",
        "output": [{
            "type": "message",
            "id": "msg_67ccd2bf17f0819081ff3bb2cf6508e60bb6a6b452d3795b",
            "status": "completed",
            "role": "assistant",
            "content": [{
                "type": "output_text",
                "text": "In a peaceful grove beneath a silver moon...",
                "annotations": []
            }]
        }],
        "parallel_tool_calls": true,
        "previous_response_id": null,
        "reasoning": {"effort": null, "summary": null},
        "store": true,
        "temperature": 1.0,
        "text": {"format": {"type": "text"}},
        "tool_choice": "auto",
        "tools": [],
        "top_p": 1.0,
        "truncation": "disabled",
        "usage": {
            "input_tokens": 36,
            "input_tokens_details": {"cached_tokens": 0},
            "output_tokens": 87,
            "output_tokens_details": {"reasoning_tokens": 0},
            "total_tokens": 123
        },
        "user": null,
        "metadata": {}
    })
}

#[test]
fn a_response_without_instructions_logprobs_or_cache_write_tokens_decodes() {
    let response: Response = serde_json::from_value(api_reference_response())
        .expect("the API reference's response decodes");
    assert!(response.instructions.is_none());
    let usage = response.usage.flatten().expect("usage");
    assert_eq!(usage.input_tokens_details.cache_write_tokens, None);
}

#[test]
fn item_input_is_sent_as_a_list() {
    let input: InputParam = serde_json::from_value(json!([
        {"role": "user", "content": "Hello"}
    ]))
    .expect("an item list is an InputParam");
    assert!(matches!(input, InputParam::InputItemList(_)));
    let request = CreateResponse {
        input: Some(input),
        ..Default::default()
    };
    let body = serde_json::to_value(&request).unwrap();
    assert_eq!(body["input"], json!([{"role": "user", "content": "Hello"}]));
}

#[test]
fn text_input_is_sent_as_a_string() {
    let request = CreateResponse {
        input: Some(InputParam::TextInput("Hello".to_owned())),
        ..Default::default()
    };
    assert_eq!(
        serde_json::to_value(&request).unwrap()["input"],
        json!("Hello")
    );
}
