// The configuration, errors and helpers, under the same paths: `super::Error` names them.
#[doc(no_inline)]
pub use openai_client_base_common::apis::*;

#[cfg(feature = "responses")]
pub mod conversations_api;
#[cfg(feature = "responses")]
pub mod responses_api;
