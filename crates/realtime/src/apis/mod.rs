// The configuration, errors and helpers, under the same paths: `super::Error` names them.
#[doc(no_inline)]
pub use openai_client_base_common::apis::*;

#[cfg(feature = "audio")]
pub mod audio_api;
#[cfg(feature = "live")]
pub mod live_api;
#[cfg(feature = "realtime")]
pub mod realtime_api;
