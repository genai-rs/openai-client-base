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
//!     bearer_access_token: Some(std::env::var("OPENAI_API_KEY")?),
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

#![allow(
    clippy::too_many_arguments,
    clippy::needless_return,
    clippy::unnecessary_operation,
    clippy::into_iter_on_ref,
    clippy::empty_docs,
    clippy::new_without_default,
    clippy::large_enum_variant,
    clippy::derive_partial_eq_without_eq,
    clippy::enum_variant_names,
    clippy::redundant_field_names,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::derivable_impls  // Rust 1.91.0 new lint - generated code has manual Default impls
)]

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_repr;
extern crate url;

pub mod apis;
pub mod models;
