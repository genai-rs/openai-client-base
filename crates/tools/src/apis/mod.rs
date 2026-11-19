// The configuration, errors and helpers, under the same paths: `super::Error` names them.
#[doc(no_inline)]
pub use openai_client_base_common::apis::*;

#[cfg(any(feature = "chatkit", feature = "container"))]
pub mod default_api;
#[cfg(feature = "skill")]
pub mod skills_api;
#[cfg(feature = "vaults")]
pub mod vaults_api;
