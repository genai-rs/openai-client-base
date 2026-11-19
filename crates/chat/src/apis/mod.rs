// The configuration, errors and helpers, under the same paths: `super::Error` names them.
#[doc(no_inline)]
pub use openai_client_base_common::apis::*;

#[cfg(feature = "chat-completion")]
pub mod chat_api;
#[cfg(feature = "completions")]
pub mod completions_api;
#[cfg(feature = "embedding")]
pub mod embeddings_api;
#[cfg(feature = "image")]
pub mod images_api;
#[cfg(feature = "model")]
pub mod models_api;
#[cfg(feature = "moderation")]
pub mod moderations_api;
