//! Mocked-HTTP smoke tests: no network needed, requests go to a local wiremock server.

use openai_client_base::apis::{
    configuration::Configuration, embeddings_api, files_api, models_api, Error,
};
use openai_client_base::models;
use serde_json::json;
use wiremock::matchers::{body_json, header, method, path};
use wiremock::ResponseTemplate;
use wiremock::{Mock, MockServer, Request};

const USER_AGENT: &str = "openai-client-base-smoke/1.0";
const TOKEN: &str = "sk-test-token";

fn config(server: &MockServer) -> Configuration {
    Configuration {
        base_path: server.uri(),
        user_agent: Some(USER_AGENT.to_owned()),
        bearer_access_token: Some(TOKEN.to_owned()),
        ..Configuration::default()
    }
}

fn file_json(id: &str) -> serde_json::Value {
    json!({
        "id": id,
        "bytes": 120000,
        "created_at": 1_700_000_000,
        "filename": "data.jsonl",
        "object": "file",
        "purpose": "fine-tune",
        "status": "processed"
    })
}

#[tokio::test]
async fn list_models_sends_bearer_and_user_agent_and_parses_body() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/models"))
        .and(header("authorization", "Bearer sk-test-token"))
        .and(header("user-agent", USER_AGENT))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "object": "list",
            "data": [{"id": "gpt-test", "object": "model", "created": 1_686_935_002, "owned_by": "openai"}]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let response = models_api::list_models()
        .configuration(&config(&server))
        .call()
        .await
        .expect("200 response should deserialise");

    assert_eq!(response.object, models::list_models_response::Object::List);
    assert_eq!(response.data.len(), 1);
    assert_eq!(response.data[0].id, "gpt-test");
    assert_eq!(response.data[0].created, 1_686_935_002);
    assert_eq!(response.data[0].owned_by, "openai");
}

#[tokio::test]
async fn list_files_encodes_query_parameters_exactly() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/files"))
        .and(header("authorization", "Bearer sk-test-token"))
        .and(header("user-agent", USER_AGENT))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "object": "list",
            "data": [file_json("file-1")],
            "first_id": "file-1",
            "last_id": "file-1",
            "has_more": false
        })))
        .expect(1)
        .mount(&server)
        .await;

    let response = files_api::list_files()
        .configuration(&config(&server))
        .purpose("fine tune&x")
        .limit(5)
        .order("desc")
        .after("file/abc")
        .call()
        .await
        .expect("200 response should deserialise");

    assert_eq!(response.data.len(), 1);
    assert_eq!(response.data[0].filename, "data.jsonl");
    assert!(!response.has_more);

    let received: Vec<Request> = server.received_requests().await.unwrap();
    assert_eq!(received.len(), 1);
    assert_eq!(
        received[0].url.query(),
        Some("purpose=fine+tune%26x&limit=5&order=desc&after=file%2Fabc")
    );
}

#[tokio::test]
async fn retrieve_file_200_parses_model() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/files/file-abc123"))
        .and(header("authorization", "Bearer sk-test-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(file_json("file-abc123")))
        .expect(1)
        .mount(&server)
        .await;

    let file = files_api::retrieve_file()
        .configuration(&config(&server))
        .file_id("file-abc123")
        .call()
        .await
        .expect("200 response should deserialise");

    assert_eq!(file.id, "file-abc123");
    assert_eq!(file.bytes, Some(120000));
    assert_eq!(file.purpose, models::open_ai_file::Purpose::FineTune);
    assert_eq!(file.status, models::open_ai_file::Status::Processed);
}

#[tokio::test]
async fn retrieve_file_404_yields_response_error_with_typed_entity() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/files/missing"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": {
                "message": "No such File object: missing",
                "type": "invalid_request_error",
                "param": "id",
                "code": null
            }
        })))
        .mount(&server)
        .await;

    let err = files_api::retrieve_file()
        .configuration(&config(&server))
        .file_id("missing")
        .call()
        .await
        .expect_err("404 must be an error");

    match err {
        Error::ResponseError(resp) => {
            assert_eq!(resp.status.as_u16(), 404);
            assert!(resp.content.contains("No such File object"));
            match resp.entity {
                Some(files_api::RetrieveFileError::Status404(e)) => {
                    assert_eq!(e.error.message, "No such File object: missing");
                    assert_eq!(e.error.r#type, "invalid_request_error");
                    assert_eq!(e.error.param.as_deref(), Some("id"));
                }
                other => panic!("unexpected entity: {other:?}"),
            }
        }
        other => panic!("expected ResponseError, got {other:?}"),
    }
}

#[tokio::test]
async fn create_embedding_posts_json_and_parses_response() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/embeddings"))
        .and(header("authorization", "Bearer sk-test-token"))
        .and(header("user-agent", USER_AGENT))
        .and(body_json(
            json!({"input": "hello", "model": "text-embedding-test"}),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "object": "list",
            "model": "text-embedding-test",
            "data": [{"object": "embedding", "index": 0, "embedding": [0.25, -0.5]}],
            "usage": {"prompt_tokens": 1, "total_tokens": 1}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let request = models::CreateEmbeddingRequest::new(
        models::CreateEmbeddingRequestInput::new_text("hello".to_owned()),
        "text-embedding-test".to_owned(),
    );
    let response = embeddings_api::create_embedding()
        .configuration(&config(&server))
        .create_embedding_request(request)
        .call()
        .await
        .expect("200 response should deserialise");

    assert_eq!(response.model, "text-embedding-test");
    assert_eq!(response.usage.total_tokens, 1);
    assert_eq!(response.data.len(), 1);
    match &*response.data[0].embedding {
        models::EmbeddingEmbedding::Arrayofnumbers(v) => {
            assert_eq!(v, &vec![json!(0.25), json!(-0.5)]);
        }
        other => panic!("unexpected embedding: {other:?}"),
    }
}
