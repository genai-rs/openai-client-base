//! Generated Rust client for the OpenAI API.
//!
//! # Examples
//!
//! Build a [`Configuration`](apis::configuration::Configuration) with a bearer
//! token and call an endpoint through its generated builder:
//!
//! ```no_run
//! use openai_client_base::apis::{configuration::Configuration, models_api};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let configuration = Configuration {
//!     base_path: "https://api.openai.com/v1".to_owned(),
//!     bearer_access_token: Some(std::env::var("OPENAI_API_KEY")?.into()),
//!     ..Configuration::default()
//! };
//!
//! let models = models_api::list_models()
//!     .configuration(&configuration)
//!     .call()
//!     .await?;
//! println!("{} models available", models.data.len());
//! # Ok(())
//! # }
//! ```

// splitCrates: the models and API modules are in the crates of the workspace (crates/), which `apis` and
// `models` re-export under these paths. The lints of every crate are in Cargo.toml ([workspace.lints]).
// Generated with apiFeatures: the API modules need the HTTP client (any API feature, or `full`).
#[cfg(feature = "_api")]
pub mod apis;
pub mod models;
