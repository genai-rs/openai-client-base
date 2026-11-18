//! The generated server's traits, implemented by the scripts.

use std::sync::{Arc, Mutex};

use axum::extract::{DefaultBodyLimit, Request};
use axum::middleware::Next;
use axum::response::Response;
use http::StatusCode;
use openai_server_base::server::{self, EventStream, RequestError, ServerError};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::reply::Reply;
use crate::state::State;

/// Every operation the mock serves.
pub const SERVED: &[&str] = &[
    "create_chat_completion",
    "create_response",
    "get_response",
    "cancel_response",
    "delete_response",
    "create_embedding",
    "list_models",
    "retrieve_model",
    "create_file",
    "list_files",
    "retrieve_file",
    "download_file",
    "delete_file",
    "create_batch",
    "retrieve_batch",
    "list_batches",
    "cancel_batch",
    "create_image",
    "create_image_edit",
    "create_speech",
    "create_transcription",
    "create_moderation",
    "create_realtime_call",
];

/// The largest request body the mock reads: file uploads in tests stay well below it.
const BODY_LIMIT: usize = 256 * 1024 * 1024;

pub(crate) fn app(state: Arc<State>) -> axum::Router {
    let api = server::router(Mock {
        state: state.clone(),
    });
    axum::Router::new()
        .nest("/v1", api.clone())
        .merge(api)
        .layer(DefaultBodyLimit::max(BODY_LIMIT))
        .layer(axum::middleware::from_fn(
            move |request: Request, next: Next| {
                let state = state.clone();
                async move { authorize(&state, request, next).await }
            },
        ))
}

/// With an API key, a request whose `Authorization` is not `Bearer <key>` is a `401`.
async fn authorize(state: &State, request: Request, next: Next) -> Response {
    if let Some(key) = &state.api_key {
        let expected = format!("Bearer {key}");
        let given = request
            .headers()
            .get(http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok());
        if given != Some(expected.as_str()) {
            return Reply::error(401, "Incorrect API key provided.").into_response();
        }
    }
    next.run(request).await
}

#[derive(Clone)]
struct Mock {
    state: Arc<State>,
}

/// A response the mock writes itself (not through the typed response), carried to [`server::ErrorHandler`] as an
/// error.
#[derive(Debug)]
struct Written(Mutex<Option<Response>>);

impl std::fmt::Display for Written {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("a scripted reply")
    }
}

impl std::error::Error for Written {}

fn written(reply: Reply) -> ServerError {
    ServerError::from(Written(Mutex::new(Some(reply.into_response()))))
}

/// The reply as the mock writes it, after its delay.
async fn delayed(reply: &Reply) {
    if let Some(delay) = reply.delay {
        tokio::time::sleep(delay).await;
    }
}

/// A fixture that does not decode as the operation's model: the test's mistake, answered with a `500` naming it.
fn fixture_error(operation: &str, error: serde_json::Error) -> ServerError {
    written(Reply::error(
        500,
        format!("openai-mock-server: the reply scripted for {operation} does not decode as its model: {error}"),
    ))
}

fn decode<T: DeserializeOwned>(operation: &str, value: Value) -> Result<T, ServerError> {
    serde_json::from_value(value).map_err(|e| fixture_error(operation, e))
}

fn decode_all<T: DeserializeOwned>(
    operation: &str,
    values: Vec<Value>,
) -> Result<Vec<T>, ServerError> {
    values.into_iter().map(|v| decode(operation, v)).collect()
}

impl Mock {
    /// Records the request and returns the operation's next reply, after its delay; no reply scripted is a `501`.
    async fn next<T: std::any::Any + Send + Sync>(
        &self,
        operation: &'static str,
        request: T,
    ) -> Result<Reply, ServerError> {
        let Some(reply) = self.state.take(operation, request) else {
            return Err(written(Reply::error(
                501,
                format!("openai-mock-server: no reply scripted for {operation}"),
            )));
        };
        delayed(&reply).await;
        Ok(reply)
    }
}

impl server::ErrorHandler for Mock {
    fn invalid_request(&self, error: RequestError) -> Response {
        let message = error.to_string();
        self.state.reject(message.clone());
        let body = crate::fixtures::error(
            "invalid_request_error",
            message,
            error.name().map(str::to_owned),
            None,
        );
        Reply::json(body)
            .status(error.status().as_u16())
            .into_response()
    }

    fn server_error(&self, error: ServerError) -> Response {
        if let Some(written) = error.inner().downcast_ref::<Written>() {
            if let Some(response) = written.0.lock().ok().and_then(|mut r| r.take()) {
                return response;
            }
        }
        if let Some(operation) = error.not_implemented_operation() {
            let body = crate::fixtures::error(
                "invalid_request_error",
                format!("openai-mock-server does not serve {operation}"),
                None,
                Some("not_implemented".to_owned()),
            );
            return Reply::json(body)
                .status(StatusCode::NOT_IMPLEMENTED.as_u16())
                .into_response();
        }
        Reply::error(500, format!("openai-mock-server: {error}")).into_response()
    }
}

/// An operation answered from its script: a plain `200` JSON reply as `$json` (decoded as its model), a plain
/// stream as `$stream`, plain bytes as `$bytes`, plain text as `$text`; anything else written as scripted.
macro_rules! scripted {
    ($name:ident($request:ty) -> $response:ty
        $(, json = $json:ident)? $(, stream = $stream:ident)? $(, bytes = $bytes:ident)? $(, text = $text:ident)?) => {
        async fn $name(&self, request: $request) -> Result<$response, ServerError> {
            type R = $response;
            const OP: &str = stringify!($name);
            let reply = self.next(OP, request).await?;
            $(if let Some(body) = reply.plain_json(StatusCode::OK) {
                return Ok(R::$json(decode(OP, body)?));
            })?
            $(if let Some(events) = reply.plain_stream() {
                return Ok(R::$stream(EventStream::from_events(decode_all(OP, events)?).done()));
            })?
            $(if let Some((bytes, content_type)) = reply.plain_bytes(StatusCode::OK) {
                return Ok(R::$bytes(server::Binary::bytes(bytes).content_type(&content_type)));
            })?
            $(if let Some(text) = reply.plain_text() {
                return Ok(R::$text(text));
            })?
            Err(written(reply))
        }
    };
}

use server::audio_api as audio;
use server::batch_api as batch;
use server::chat_api as chat;
use server::embeddings_api as embeddings;
use server::files_api as files;
use server::images_api as images;
use server::models_api as models;
use server::moderations_api as moderations;
use server::realtime_api as realtime;
use server::responses_api as responses;

impl server::ChatApi for Mock {
    scripted!(create_chat_completion(chat::CreateChatCompletionRequest) -> chat::CreateChatCompletionResponse,
        json = Status200, stream = Status200Stream);
}

impl server::ResponsesApi for Mock {
    scripted!(create_response(responses::CreateResponseRequest) -> responses::CreateResponseResponse,
        json = Status200, stream = Status200Stream);
    scripted!(get_response(responses::GetResponseRequest) -> responses::GetResponseResponse,
        json = Status200, stream = Status200Stream);
    scripted!(cancel_response(responses::CancelResponseRequest) -> responses::CancelResponseResponse,
        json = Status200);
    scripted!(delete_response(responses::DeleteResponseRequest) -> responses::DeleteResponseResponse,
        json = Status200);
}

impl server::EmbeddingsApi for Mock {
    scripted!(create_embedding(embeddings::CreateEmbeddingRequest) -> embeddings::CreateEmbeddingResponse,
        json = Status200);
}

impl server::ModelsApi for Mock {
    scripted!(list_models(models::ListModelsRequest) -> models::ListModelsResponse, json = Status200);
    scripted!(retrieve_model(models::RetrieveModelRequest) -> models::RetrieveModelResponse, json = Status200);
}

impl server::FilesApi for Mock {
    scripted!(create_file(files::CreateFileRequest) -> files::CreateFileResponse, json = Status200);
    scripted!(list_files(files::ListFilesRequest) -> files::ListFilesResponse, json = Status200);
    scripted!(retrieve_file(files::RetrieveFileRequest) -> files::RetrieveFileResponse, json = Status200);
    scripted!(download_file(files::DownloadFileRequest) -> files::DownloadFileResponse, bytes = Status200);
    scripted!(delete_file(files::DeleteFileRequest) -> files::DeleteFileResponse, json = Status200);
}

impl server::BatchApi for Mock {
    scripted!(create_batch(batch::CreateBatchRequest) -> batch::CreateBatchResponse, json = Status200);
    scripted!(retrieve_batch(batch::RetrieveBatchRequest) -> batch::RetrieveBatchResponse, json = Status200);
    scripted!(list_batches(batch::ListBatchesRequest) -> batch::ListBatchesResponse, json = Status200);
    scripted!(cancel_batch(batch::CancelBatchRequest) -> batch::CancelBatchResponse, json = Status200);
}

impl server::ImagesApi for Mock {
    scripted!(create_image(images::CreateImageRequest) -> images::CreateImageResponse,
        json = Status200, stream = Status200Stream);
    scripted!(create_image_edit(images::CreateImageEditRequest) -> images::CreateImageEditResponse,
        json = Status200, stream = Status200Stream);
}

impl server::AudioApi for Mock {
    scripted!(create_speech(audio::CreateSpeechRequest) -> audio::CreateSpeechResponse,
        stream = Status200Stream, bytes = Status200);
    scripted!(create_transcription(audio::CreateTranscriptionRequest) -> audio::CreateTranscriptionResponse,
        json = Status200, stream = Status200Stream, text = Status200Text);
}

impl server::ModerationsApi for Mock {
    scripted!(create_moderation(moderations::CreateModerationRequest) -> moderations::CreateModerationResponse,
        json = Status200);
}

impl server::RealtimeApi for Mock {
    async fn create_realtime_call(
        &self,
        request: realtime::CreateRealtimeCallRequest,
    ) -> Result<realtime::CreateRealtimeCallResponse, ServerError> {
        let reply = self.next("create_realtime_call", request).await?;
        if let Some((bytes, content_type)) = reply.plain_bytes(StatusCode::CREATED) {
            return Ok(realtime::CreateRealtimeCallResponse::Status201 {
                body: server::Binary::bytes(bytes).content_type(&content_type),
                headers: Default::default(),
            });
        }
        Err(written(reply))
    }
}

// Every other operation answers `501` (the generated default methods).
impl server::AgentsApi for Mock {}
impl server::AssistantsApi for Mock {}
impl server::AuditLogsApi for Mock {}
impl server::CertificatesApi for Mock {}
impl server::CompletionsApi for Mock {}
impl server::ConversationsApi for Mock {}
impl server::DataRetentionApi for Mock {}
impl server::DecisionsApi for Mock {}
impl server::DefaultApi for Mock {}
impl server::EvalsApi for Mock {}
impl server::FineTuningApi for Mock {}
impl server::GroupOrganizationRoleAssignmentsApi for Mock {}
impl server::GroupUsersApi for Mock {}
impl server::GroupsApi for Mock {}
impl server::HostedToolsApi for Mock {}
impl server::InvitesApi for Mock {}
impl server::LiveApi for Mock {}
impl server::ProjectGroupRoleAssignmentsApi for Mock {}
impl server::ProjectGroupsApi for Mock {}
impl server::ProjectUserRoleAssignmentsApi for Mock {}
impl server::ProjectsApi for Mock {}
impl server::RolesApi for Mock {}
impl server::SkillsApi for Mock {}
impl server::SpendAlertsApi for Mock {}
impl server::UploadsApi for Mock {}
impl server::UsageApi for Mock {}
impl server::UserOrganizationRoleAssignmentsApi for Mock {}
impl server::UsersApi for Mock {}
impl server::VaultsApi for Mock {}
impl server::VectorStoresApi for Mock {}
impl server::VideosApi for Mock {}
