// The configuration, errors and helpers, under the same paths: `super::Error` names them.
#[doc(no_inline)]
pub use openai_client_base_common::apis::*;

#[cfg(feature = "evals")]
pub mod evals_api;
#[cfg(feature = "finetuning")]
pub mod fine_tuning_api;
