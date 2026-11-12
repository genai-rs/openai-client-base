//! The typed server-sent event streams (`typedEventStreams`) and the per-media-type methods (`contentTypeMethods`)
//! against a mocked server: each `data:` payload decodes to the operation's event type by its `type`, `[DONE]` ends
//! the stream, and errors keep the operation's error type.

use futures_util::StreamExt;
use openai_client_base::apis::{chat_api, configuration::Configuration, responses_api, sse, Error};
use openai_client_base::models;
use serde_json::json;
use wiremock::matchers::{body_partial_json, header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn config(server: &MockServer) -> Configuration {
    Configuration {
        base_path: server.uri(),
        bearer_access_token: Some("sk-test".to_owned()),
        ..Configuration::default()
    }
}

fn sse_body(events: &[(&str, serde_json::Value)]) -> String {
    let mut body = String::new();
    for (name, data) in events {
        body.push_str(&format!("event: {name}\ndata: {data}\n\n"));
    }
    body
}

fn event_stream(body: String) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_raw(body, "text/event-stream")
}

fn streaming_request() -> models::CreateResponse {
    let mut request = models::CreateResponse::new();
    request.model = Some("gpt-test".to_owned());
    request.stream = Some(Some(true));
    request
}

#[tokio::test]
async fn create_response_stream_yields_typed_events() {
    let server = MockServer::start().await;
    let body = sse_body(&[
        (
            "response.output_item.added",
            json!({
                "type": "response.output_item.added", "output_index": 0, "sequence_number": 1,
                "item": {"type": "function_call", "id": "fc_1", "call_id": "call_1", "name": "get_weather",
                         "arguments": "", "status": "in_progress"}
            }),
        ),
        // The event name is not what selects the variant: the payload's type is.
        (
            "message",
            json!({
                "type": "response.output_text.delta", "item_id": "msg_1", "output_index": 0,
                "content_index": 0, "delta": "Hel", "sequence_number": 2, "logprobs": []
            }),
        ),
        (
            "response.output_text.delta",
            json!({
                "type": "response.output_text.delta", "item_id": "msg_1", "output_index": 0,
                "content_index": 0, "delta": "lo", "sequence_number": 3, "logprobs": []
            }),
        ),
        (
            "response.something.new",
            json!({"type": "response.something.new", "sequence_number": 4}),
        ),
    ]);
    Mock::given(method("POST"))
        .and(path("/responses"))
        .and(header("accept", "text/event-stream"))
        .and(header("authorization", "Bearer sk-test"))
        .and(body_partial_json(json!({"stream": true})))
        .respond_with(event_stream(body))
        .expect(1)
        .mount(&server)
        .await;

    let events: Vec<models::ResponseStreamEvent> = responses_api::create_response_stream()
        .configuration(&config(&server))
        .create_response(&streaming_request())
        .call()
        .await
        .expect("an event stream")
        .map(|event| event.expect("every event decodes"))
        .collect()
        .await;

    assert_eq!(events.len(), 4);
    match &events[0] {
        models::ResponseStreamEvent::ResponseOutputItemAddedEvent(added) => match &added.item {
            models::OutputItem::FunctionToolCall(call) => assert_eq!(call.name, "get_weather"),
            other => panic!("{other:?}"),
        },
        other => panic!("{other:?}"),
    }
    let text: String = events
        .iter()
        .filter_map(|event| match event {
            models::ResponseStreamEvent::ResponseTextDeltaEvent(delta) => {
                Some(delta.delta.as_str())
            }
            _ => None,
        })
        .collect();
    assert_eq!(text, "Hello");
    assert!(matches!(events[3], models::ResponseStreamEvent::Unknown(_)));
}

#[tokio::test]
async fn get_response_stream_resumes_with_query_parameters() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/responses/resp_1"))
        .and(query_param("stream", "true"))
        .and(query_param("starting_after", "2"))
        .respond_with(event_stream(sse_body(&[(
            "response.output_text.delta",
            json!({"type": "response.output_text.delta", "item_id": "msg_1", "output_index": 0,
                   "content_index": 0, "delta": "!", "sequence_number": 3, "logprobs": []}),
        )])))
        .expect(1)
        .mount(&server)
        .await;

    let events: Vec<_> = responses_api::get_response_stream()
        .configuration(&config(&server))
        .response_id("resp_1")
        .stream(true)
        .starting_after(2)
        .call()
        .await
        .unwrap()
        .collect()
        .await;
    assert_eq!(events.len(), 1);
    assert!(
        matches!(&events[0], Ok(models::ResponseStreamEvent::ResponseTextDeltaEvent(d)) if d.delta == "!")
    );
}

#[tokio::test]
async fn chat_completion_stream_stops_at_done() {
    let server = MockServer::start().await;
    let chunk = |content: &str| {
        json!({"id": "chatcmpl-1", "object": "chat.completion.chunk", "created": 1, "model": "gpt-test",
               "choices": [{"index": 0, "delta": {"content": content}, "finish_reason": null}]})
    };
    let body = format!(
        "data: {}\n\n: keep-alive\n\ndata: {}\n\ndata: [DONE]\n\ndata: {}\n\n",
        chunk("Hel"),
        chunk("lo"),
        chunk("after done")
    );
    Mock::given(method("POST"))
        .and(path("/chat/completions"))
        .respond_with(event_stream(body))
        .mount(&server)
        .await;

    let mut request = models::CreateChatCompletionRequest::new(vec![], "gpt-test".to_owned());
    request.stream = Some(Some(true));
    let chunks: Vec<models::CreateChatCompletionStreamResponse> =
        chat_api::create_chat_completion_stream()
            .configuration(&config(&server))
            .create_chat_completion_request(&request)
            .call()
            .await
            .unwrap()
            .map(Result::unwrap)
            .collect()
            .await;
    let text: String = chunks
        .iter()
        .filter_map(|c| c.choices[0].delta.content.clone().flatten())
        .collect();
    assert_eq!(text, "Hello");
}

#[tokio::test]
async fn an_error_status_is_the_operations_typed_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/responses"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "error": {"message": "stream is not supported for this model", "type": "invalid_request_error",
                      "param": "stream", "code": null}
        })))
        .mount(&server)
        .await;

    let error = responses_api::create_response_stream()
        .configuration(&config(&server))
        .create_response(&streaming_request())
        .call()
        .await
        .unwrap_err();
    let Error::ResponseError(content) = error else {
        panic!("{error:?}")
    };
    assert_eq!(content.status.as_u16(), 400);
    match content.entity {
        Some(responses_api::CreateResponseError::Status400(body)) => {
            assert_eq!(body.error.message, "stream is not supported for this model")
        }
        other => panic!("{other:?}"),
    }
}

#[tokio::test]
async fn a_json_answer_to_a_stream_request_is_an_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/responses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": "resp_1"})))
        .mount(&server)
        .await;

    let error = responses_api::create_response_stream()
        .configuration(&config(&server))
        .create_response(&streaming_request())
        .call()
        .await
        .unwrap_err();
    assert!(
        matches!(&error, Error::Serde(e) if e.to_string().contains("application/json")),
        "{error:?}"
    );
}

#[tokio::test]
async fn a_payload_that_does_not_decode_is_an_error_and_the_stream_goes_on() {
    let server = MockServer::start().await;
    let body = "data: {\"type\":\"response.output_text.delta\"}\n\n\
                data: {\"type\":\"response.output_text.delta\",\"item_id\":\"m\",\"output_index\":0,\
                \"content_index\":0,\"delta\":\"ok\",\"sequence_number\":1,\"logprobs\":[]}\n\n";
    Mock::given(method("POST"))
        .and(path("/responses"))
        .respond_with(event_stream(body.to_owned()))
        .mount(&server)
        .await;

    let events: Vec<_> = responses_api::create_response_stream()
        .configuration(&config(&server))
        .create_response(&streaming_request())
        .call()
        .await
        .unwrap()
        .collect()
        .await;
    assert_eq!(events.len(), 2);
    assert!(matches!(events[0], Err(Error::Serde(_))));
    assert!(
        matches!(&events[1], Ok(models::ResponseStreamEvent::ResponseTextDeltaEvent(d)) if d.delta == "ok")
    );
}

#[tokio::test]
async fn the_raw_events_keep_names_and_payloads() {
    let body = "event: response.created\nid: 1\ndata: {\"type\":\"x\"}\n\ndata: [DONE]\n\n";
    let chunks = vec![Ok::<_, timvw_rust_runtime::Error>(body.as_bytes())];
    let events: Vec<sse::Event> = sse::Events::from_bytes(futures_util::stream::iter(chunks))
        .map(Result::unwrap)
        .collect()
        .await;
    assert_eq!(
        events,
        vec![sse::Event {
            event: Some("response.created".to_owned()),
            data: "{\"type\":\"x\"}".to_owned(),
            id: Some("1".to_owned()),
        }]
    );
}
