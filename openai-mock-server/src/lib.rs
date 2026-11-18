//! A scripted mock of the OpenAI API for tests.
//!
//! [`MockServer`] serves the OpenAI API on `127.0.0.1` with the server generated from OpenAI's spec
//! ([`openai-server-base`](openai_server_base)), whose models are [`openai-client-base`](https://docs.rs/openai-client-base)'s.
//! A test scripts what each operation answers, points its client at [`MockServer::base_url`], and reads back the
//! requests the server decoded, as typed values:
//!
//! ```no_run
//! # async fn example() {
//! use openai_mock_server::{fixtures, op, MockServer, Reply};
//! use openai_mock_server::server::chat_api::CreateChatCompletionRequest;
//!
//! let mock = MockServer::start().await;
//! mock.on(op::CREATE_CHAT_COMPLETION).reply(Reply::json(fixtures::chat_completion("Hello!")));
//!
//! // ... call `mock.base_url()` (`http://127.0.0.1:<port>/v1`) with the client under test ...
//!
//! let requests = mock.requests::<CreateChatCompletionRequest>();
//! assert_eq!(requests[0].body.model, "gpt-4o-mini");
//! # }
//! ```
//!
//! - **Scripts** are per operation, a queue of [`Reply`]s taken in order, then the [`always`](Script::always) reply
//!   if one is set. A reply is JSON, an event [`Stream`], bytes, text, an error in OpenAI's envelope, or a `429`
//!   with `retry-after`, optionally after a delay.
//! - **Typed:** a plain `200` goes through the generated server's typed response, so a fixture that does not decode
//!   as the operation's model fails the request with a `500` that names the operation, instead of passing
//!   silently. Requests are decoded by the generated server too: one that does not decode is answered with a `400`
//!   in OpenAI's envelope and listed by [`MockServer::rejected`].
//! - **Served:** the operations of [`op`]: chat completions and responses (JSON and streams), embeddings, models,
//!   files, batches, images, audio, moderations and the realtime call. Every other operation of the spec answers
//!   `501` in OpenAI's envelope.

mod api;
pub mod fixtures;
mod reply;
mod state;

use std::any::Any;
use std::net::SocketAddr;
use std::sync::Arc;

pub use openai_server_base::{models, server};
pub use reply::{Reply, Stream, StreamError};

use state::State;

/// The operations the mock serves, by the name of their generated method (and their `operationId`).
pub mod op {
    pub use crate::api::SERVED;

    /// `POST /chat/completions`
    pub const CREATE_CHAT_COMPLETION: &str = "create_chat_completion";
    /// `POST /responses`
    pub const CREATE_RESPONSE: &str = "create_response";
    /// `GET /responses/{response_id}`
    pub const GET_RESPONSE: &str = "get_response";
    /// `POST /responses/{response_id}/cancel`
    pub const CANCEL_RESPONSE: &str = "cancel_response";
    /// `DELETE /responses/{response_id}`
    pub const DELETE_RESPONSE: &str = "delete_response";
    /// `POST /embeddings`
    pub const CREATE_EMBEDDING: &str = "create_embedding";
    /// `GET /models`
    pub const LIST_MODELS: &str = "list_models";
    /// `GET /models/{model}`
    pub const RETRIEVE_MODEL: &str = "retrieve_model";
    /// `POST /files` (multipart)
    pub const CREATE_FILE: &str = "create_file";
    /// `GET /files`
    pub const LIST_FILES: &str = "list_files";
    /// `GET /files/{file_id}`
    pub const RETRIEVE_FILE: &str = "retrieve_file";
    /// `GET /files/{file_id}/content`
    pub const DOWNLOAD_FILE: &str = "download_file";
    /// `DELETE /files/{file_id}`
    pub const DELETE_FILE: &str = "delete_file";
    /// `POST /batches`
    pub const CREATE_BATCH: &str = "create_batch";
    /// `GET /batches/{batch_id}`
    pub const RETRIEVE_BATCH: &str = "retrieve_batch";
    /// `GET /batches`
    pub const LIST_BATCHES: &str = "list_batches";
    /// `POST /batches/{batch_id}/cancel`
    pub const CANCEL_BATCH: &str = "cancel_batch";
    /// `POST /images/generations`
    pub const CREATE_IMAGE: &str = "create_image";
    /// `POST /images/edits` (JSON or multipart)
    pub const CREATE_IMAGE_EDIT: &str = "create_image_edit";
    /// `POST /audio/speech`
    pub const CREATE_SPEECH: &str = "create_speech";
    /// `POST /audio/transcriptions` (multipart)
    pub const CREATE_TRANSCRIPTION: &str = "create_transcription";
    /// `POST /moderations`
    pub const CREATE_MODERATION: &str = "create_moderation";
    /// `POST /realtime/calls` (SDP or multipart)
    pub const CREATE_REALTIME_CALL: &str = "create_realtime_call";
}

/// The mock: an OpenAI API on a local port, answering what the test scripts. Stops when dropped.
pub struct MockServer {
    state: Arc<State>,
    address: SocketAddr,
    task: tokio::task::JoinHandle<()>,
}

impl MockServer {
    /// Starts the mock on `127.0.0.1`, on a port the system picks. Needs a Tokio runtime.
    ///
    /// # Panics
    ///
    /// When no local port can be bound.
    pub async fn start() -> Self {
        MockServer::start_with(State::new(None)).await
    }

    /// Starts the mock and answers `401` (in OpenAI's envelope) to a request whose `Authorization` is not
    /// `Bearer <api_key>`.
    ///
    /// # Panics
    ///
    /// When no local port can be bound.
    pub async fn with_api_key(api_key: impl Into<String>) -> Self {
        MockServer::start_with(State::new(Some(api_key.into()))).await
    }

    async fn start_with(state: State) -> Self {
        let state = Arc::new(state);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("a local port for the mock");
        let address = listener.local_addr().expect("the mock's address");
        let app = api::app(state.clone());
        let task = tokio::spawn(async move {
            let _ = axum::serve(listener, app).await;
        });
        MockServer {
            state,
            address,
            task,
        }
    }

    /// The API's base URL, `http://127.0.0.1:<port>/v1`. The paths are served without the `/v1` prefix as well.
    pub fn base_url(&self) -> String {
        format!("http://{}/v1", self.address)
    }

    /// The address the mock listens on.
    pub fn address(&self) -> SocketAddr {
        self.address
    }

    /// The script of `operation` (one of [`op`]), to add replies to.
    ///
    /// # Panics
    ///
    /// When the mock does not serve `operation`: scripting it would have no effect.
    pub fn on(&self, operation: &'static str) -> Script<'_> {
        assert!(
            op::SERVED.contains(&operation),
            "openai-mock-server does not serve {operation}; it serves {:?}",
            op::SERVED
        );
        Script {
            state: &self.state,
            operation,
        }
    }

    /// The requests of type `T` the mock received, in order: the generated request struct of an operation, with its
    /// decoded body and its headers (e.g. `server::chat_api::CreateChatCompletionRequest`).
    pub fn requests<T: Any + Clone>(&self) -> Vec<T> {
        self.state.requests()
    }

    /// How many requests of `operation` the mock answered.
    pub fn calls(&self, operation: &str) -> usize {
        self.state.calls(operation)
    }

    /// The requests the generated server could not decode (answered with a `400` or `415`), as messages naming
    /// what did not decode. Empty when the client sent what the spec describes.
    pub fn rejected(&self) -> Vec<String> {
        self.state.rejected()
    }

    /// Forgets the scripts and the recorded requests.
    pub fn reset(&self) {
        self.state.reset();
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        self.task.abort();
    }
}

impl std::fmt::Debug for MockServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MockServer")
            .field("address", &self.address)
            .finish()
    }
}

/// The replies of one operation: see [`MockServer::on`].
#[derive(Debug)]
pub struct Script<'a> {
    state: &'a State,
    operation: &'static str,
}

impl Script<'_> {
    /// Answers the next request with `reply` (after the replies queued before it).
    pub fn reply(self, reply: Reply) -> Self {
        self.state.push(self.operation, reply);
        self
    }

    /// Answers with `reply` once the queued replies are used up, as often as asked.
    pub fn always(self, reply: Reply) -> Self {
        self.state.always(self.operation, reply);
        self
    }
}
