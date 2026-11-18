//! openai-client-base's generated client against the mock: what it sends is what the server decodes, and what
//! the mock answers is what the client reads.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::time::{Duration, Instant};

use futures_util::StreamExt;
use openai_client_base::apis::configuration::Configuration;
use openai_client_base::apis::{
    chat_api, embeddings_api, files_api, models_api, moderations_api, realtime_api, responses_api,
    Error,
};
use openai_client_base::models;
use openai_mock_server::server::{
    chat_api as chat, files_api as files, responses_api as responses,
};
use openai_mock_server::{fixtures, op, MockServer, Reply, Stream, StreamError};
use serde_json::json;

/// A client of the mock, without retries: each request takes one scripted reply.
fn config(mock: &MockServer) -> Configuration {
    Configuration {
        base_path: mock.base_url(),
        bearer_access_token: Some("sk-test".into()),
        client: timvw_rust_runtime::Client::from_service(timvw_rust_runtime::ReqwestService::new(
            reqwest::Client::new(),
        )),
        ..Configuration::default()
    }
}

fn chat_request(stream: bool) -> models::CreateChatCompletionRequest {
    serde_json::from_value(json!({
        "model": fixtures::MODEL,
        "messages": [{"role": "user", "content": "Hi"}],
        "stream": stream,
    }))
    .unwrap()
}

fn response_request() -> models::CreateResponse {
    serde_json::from_value(json!({"model": fixtures::MODEL, "input": "Hi"})).unwrap()
}

#[tokio::test]
async fn a_chat_completion_and_the_request_the_server_decoded() {
    let mock = MockServer::start().await;
    mock.on(op::CREATE_CHAT_COMPLETION)
        .reply(Reply::json(fixtures::chat_completion("Hello!")));

    let completion = chat_api::create_chat_completion()
        .configuration(&config(&mock))
        .create_chat_completion_request(&chat_request(false))
        .call()
        .await
        .unwrap();
    assert_eq!(
        completion.choices[0].message.content.as_deref(),
        Some("Hello!")
    );

    let requests = mock.requests::<chat::CreateChatCompletionRequest>();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].body.model, fixtures::MODEL);
    assert_eq!(requests[0].headers["authorization"], "Bearer sk-test");
    assert_eq!(mock.calls(op::CREATE_CHAT_COMPLETION), 1);
    assert!(mock.rejected().is_empty());
}

#[tokio::test]
async fn a_chat_stream_through_the_typed_event_stream() {
    let mock = MockServer::start().await;
    mock.on(op::CREATE_CHAT_COMPLETION)
        .reply(Reply::stream(Stream::new(fixtures::chat_chunks(&[
            "Hel", "lo",
        ]))));

    let text: String = chat_api::create_chat_completion_stream()
        .configuration(&config(&mock))
        .create_chat_completion_request(&chat_request(true))
        .call()
        .await
        .unwrap()
        .map(|chunk| {
            chunk.unwrap().choices[0]
                .delta
                .content
                .clone()
                .flatten()
                .unwrap_or_default()
        })
        .collect()
        .await;
    assert_eq!(text, "Hello");
}

#[tokio::test]
async fn a_chat_stream_with_keep_alives_and_an_error_after_it_started() {
    let mock = MockServer::start().await;
    mock.on(op::CREATE_CHAT_COMPLETION).reply(Reply::stream(
        Stream::new(fixtures::chat_chunks(&["Hel", "lo"]))
            .keep_alive()
            .error_after(2, StreamError::Data("the model is overloaded".into())),
    ));

    let events: Vec<_> = chat_api::create_chat_completion_stream()
        .configuration(&config(&mock))
        .create_chat_completion_request(&chat_request(true))
        .call()
        .await
        .unwrap()
        .collect()
        .await;
    // the keep-alive comments are skipped; the error frame ends the stream with an error
    assert_eq!(events.len(), 3, "{events:?}");
    assert!(events[0].is_ok() && events[1].is_ok());
    assert!(events[2].is_err(), "{:?}", events[2]);
}

#[tokio::test]
async fn a_response_and_a_named_response_stream() {
    let mock = MockServer::start().await;
    mock.on(op::CREATE_RESPONSE)
        .reply(Reply::json(fixtures::response("Hello!")))
        .reply(Reply::stream(Stream::new(fixtures::response_events(&[
            "Hel", "lo",
        ]))));

    let response = responses_api::create_response()
        .configuration(&config(&mock))
        .create_response(&response_request())
        .call()
        .await
        .unwrap();
    assert_eq!(response.id, "resp_mock");

    let events: Vec<models::ResponseStreamEvent> = responses_api::create_response_stream()
        .configuration(&config(&mock))
        .create_response(&response_request())
        .call()
        .await
        .unwrap()
        .map(Result::unwrap)
        .collect()
        .await;
    assert_eq!(events.len(), 5);
    assert_eq!(mock.requests::<responses::CreateResponseRequest>().len(), 2);
}

#[tokio::test]
async fn a_rate_limit_carries_retry_after() {
    let mock = MockServer::start().await;
    mock.on(op::CREATE_EMBEDDING)
        .reply(Reply::rate_limited(Duration::from_secs(3)));

    let request: models::CreateEmbeddingRequest =
        serde_json::from_value(json!({"model": "text-embedding-3-small", "input": "hi"})).unwrap();
    let error = embeddings_api::create_embedding()
        .configuration(&config(&mock))
        .create_embedding_request(&request)
        .call()
        .await
        .unwrap_err();
    let Error::ResponseError(content) = error else {
        panic!("{error:?}")
    };
    assert_eq!(content.status, 429);
    assert_eq!(content.headers["retry-after"], "3");
    assert!(
        content.content.contains("rate_limit_error"),
        "{}",
        content.content
    );
}

#[tokio::test]
async fn queued_replies_then_the_always_reply() {
    let mock = MockServer::start().await;
    mock.on(op::LIST_MODELS)
        .reply(Reply::error(500, "boom"))
        .always(Reply::json(fixtures::models(&["gpt-4o"])));

    let config = config(&mock);
    assert!(models_api::list_models()
        .configuration(&config)
        .call()
        .await
        .is_err());
    for _ in 0..2 {
        let list = models_api::list_models()
            .configuration(&config)
            .call()
            .await
            .unwrap();
        assert_eq!(list.data[0].id, "gpt-4o");
    }
    assert_eq!(mock.calls(op::LIST_MODELS), 3);
}

#[tokio::test]
async fn a_file_upload_is_recorded_with_its_bytes() {
    let mock = MockServer::start().await;
    mock.on(op::CREATE_FILE)
        .reply(Reply::json(fixtures::file("file-1", "data.jsonl", "batch")));
    mock.on(op::DOWNLOAD_FILE)
        .reply(Reply::bytes("application/jsonl", "{\"a\":1}\n"));

    let config = config(&mock);
    let file = files_api::create_file()
        .configuration(&config)
        .file(timvw_rust_runtime::Upload::bytes("{\"a\":1}\n").file_name("data.jsonl"))
        .purpose("batch")
        .call()
        .await
        .unwrap();
    assert_eq!(file.id, "file-1");
    let uploads = mock.requests::<files::CreateFileRequest>();
    assert_eq!(uploads[0].body.purpose, "batch");
    assert_eq!(
        uploads[0].body.file.file_name.as_deref(),
        Some("data.jsonl")
    );
    assert_eq!(&uploads[0].body.file.bytes[..], b"{\"a\":1}\n");

    let content = files_api::download_file()
        .configuration(&config)
        .file_id("file-1")
        .call()
        .await
        .unwrap();
    assert_eq!(content.headers()["content-type"], "application/jsonl");
    assert_eq!(content.text().await.unwrap(), "{\"a\":1}\n");
}

#[tokio::test]
async fn a_realtime_call_answers_sdp() {
    let mock = MockServer::start().await;
    mock.on(op::CREATE_REALTIME_CALL)
        .reply(Reply::sdp(fixtures::SDP_ANSWER));

    let answer = realtime_api::create_realtime_call_sdp()
        .configuration(&config(&mock))
        .body("v=0\r\n")
        .call()
        .await
        .unwrap();
    assert_eq!(answer, fixtures::SDP_ANSWER);
}

#[tokio::test]
async fn an_unserved_operation_and_an_unscripted_one_are_501() {
    let mock = MockServer::start().await;
    let config = config(&mock);
    let error = models_api::list_models()
        .configuration(&config)
        .call()
        .await
        .unwrap_err();
    let Error::ResponseError(content) = error else {
        panic!("{error:?}")
    };
    assert_eq!(content.status, 501);
    assert!(
        content
            .content
            .contains("no reply scripted for list_models"),
        "{}",
        content.content
    );

    let request: models::CreateModerationRequest =
        serde_json::from_value(json!({"input": "hi"})).unwrap();
    mock.on(op::CREATE_MODERATION)
        .reply(Reply::json(fixtures::moderation(false)));
    let moderation = moderations_api::create_moderation()
        .configuration(&config)
        .create_moderation_request(&request)
        .call()
        .await
        .unwrap();
    assert!(!moderation.results[0].flagged);
}

#[tokio::test]
async fn a_delay_and_an_api_key() {
    let mock = MockServer::with_api_key("sk-test").await;
    mock.on(op::LIST_MODELS)
        .always(Reply::json(fixtures::models(&["gpt-4o"])).delay(Duration::from_millis(200)));

    let started = Instant::now();
    models_api::list_models()
        .configuration(&config(&mock))
        .call()
        .await
        .unwrap();
    assert!(started.elapsed() >= Duration::from_millis(200));

    let wrong = Configuration {
        bearer_access_token: Some("sk-wrong".into()),
        ..config(&mock)
    };
    let error = models_api::list_models()
        .configuration(&wrong)
        .call()
        .await
        .unwrap_err();
    let Error::ResponseError(content) = error else {
        panic!("{error:?}")
    };
    assert_eq!(content.status, 401);
}

#[tokio::test]
async fn a_fixture_that_does_not_match_the_model_fails_loudly() {
    let mock = MockServer::start().await;
    mock.on(op::LIST_MODELS)
        .reply(Reply::json(json!({"object": "list", "data": "not a list"})));
    let error = models_api::list_models()
        .configuration(&config(&mock))
        .call()
        .await
        .unwrap_err();
    let Error::ResponseError(content) = error else {
        panic!("{error:?}")
    };
    assert_eq!(content.status, 500);
    assert!(
        content.content.contains("does not decode as its model"),
        "{}",
        content.content
    );
}
