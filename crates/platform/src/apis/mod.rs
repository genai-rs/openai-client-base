// The configuration, errors and helpers, under the same paths: `super::Error` names them.
#[doc(no_inline)]
pub use openai_client_base_common::apis::*;

#[cfg(feature = "decisions")]
pub mod decisions_api;
#[cfg(any(
    feature = "content-provenance-checks",
    feature = "open-ai-client-base-webhooks",
    feature = "safety",
    feature = "webhook"
))]
pub mod default_api;
#[cfg(feature = "open-ai-client-base-webhooks")]
pub mod open_ai_client_base_webhooks_api;
#[cfg(feature = "video")]
pub mod videos_api;
