// splitCrates: the API modules of every crate of the workspace, under one path.
#![allow(unused_imports)]

#[cfg(feature = "administration")]
pub use openai_client_base_administration::apis::*;
#[cfg(feature = "agents")]
pub use openai_client_base_agents::apis::*;
#[cfg(any(
    feature = "assistants",
    feature = "batch",
    feature = "file",
    feature = "upload",
    feature = "vectorstore"
))]
pub use openai_client_base_assistants::apis::*;
#[cfg(any(
    feature = "chat-completion",
    feature = "completions",
    feature = "embedding",
    feature = "image",
    feature = "model",
    feature = "moderation"
))]
pub use openai_client_base_chat::apis::*;
pub use openai_client_base_common::apis::*;
#[cfg(any(feature = "evals", feature = "finetuning"))]
pub use openai_client_base_finetuning::apis::*;
#[cfg(any(
    feature = "content-provenance-checks",
    feature = "decisions",
    feature = "open-ai-client-base-webhooks",
    feature = "safety",
    feature = "video",
    feature = "webhook"
))]
pub use openai_client_base_platform::apis::*;
#[cfg(any(feature = "audio", feature = "live", feature = "realtime"))]
pub use openai_client_base_realtime::apis::*;
#[cfg(feature = "responses")]
pub use openai_client_base_responses::apis::*;
#[cfg(any(
    feature = "chatkit",
    feature = "container",
    feature = "skill",
    feature = "vaults"
))]
pub use openai_client_base_tools::apis::*;

#[cfg(any(
    feature = "administration",
    feature = "chatkit",
    feature = "container",
    feature = "content-provenance-checks",
    feature = "open-ai-client-base-webhooks",
    feature = "safety",
    feature = "webhook"
))]
pub mod default_api {
    #[cfg(feature = "administration")]
    pub use openai_client_base_administration::apis::default_api::*;
    #[cfg(any(
        feature = "content-provenance-checks",
        feature = "open-ai-client-base-webhooks",
        feature = "safety",
        feature = "webhook"
    ))]
    pub use openai_client_base_platform::apis::default_api::*;
    #[cfg(any(feature = "chatkit", feature = "container"))]
    pub use openai_client_base_tools::apis::default_api::*;
}
