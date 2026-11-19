// The configuration, errors and helpers, under the same paths: `super::Error` names them.
#[doc(no_inline)]
pub use openai_client_base_common::apis::*;

#[cfg(feature = "assistants")]
pub mod assistants_api;
#[cfg(feature = "batch")]
pub mod batch_api;
#[cfg(feature = "file")]
pub mod files_api;
#[cfg(feature = "upload")]
pub mod uploads_api;
#[cfg(feature = "vectorstore")]
pub mod vector_stores_api;
