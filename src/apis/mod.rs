use std::error;
use std::fmt;

/// Extra headers and query parameters for one call: `.options(&options)` on an operation's builder.
pub use timvw_rust_runtime::RequestOptions;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: timvw_rust_runtime::StatusCode,
    pub content: String,
    pub entity: Option<T>,
    /// The response's headers (`retry-after`, `x-request-id`, rate limits, ...).
    pub headers: timvw_rust_runtime::header::HeaderMap,
}

#[derive(Debug)]
pub enum Error<T> {
    /// Building the request, sending it through the client's service, or reading the response body failed.
    Http(timvw_rust_runtime::Error),
    Serde(serde_json::Error),
    ResponseError(Box<ResponseContent<T>>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Http(e) => ("http", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Http(e) => e,
            Error::Serde(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<timvw_rust_runtime::Error> for Error<T> {
    fn from(e: timvw_rust_runtime::Error) -> Self {
        Error::Http(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

/// Percent-encodes a path segment: a space is `%20` (form encoding's `+` would be a literal plus in a path).
pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes())
        .collect::<String>()
        .replace('+', "%20")
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                }
                serde_json::Value::String(s) => {
                    params.push((format!("{}[{}]", prefix, key), s.clone()))
                }
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    // Non-object values (e.g. array items) are emitted as a plain key/value pair.
    let value = match value {
        serde_json::Value::String(s) => s.clone(),
        _ => value.to_string(),
    };
    vec![(prefix.to_string(), value)]
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String),
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            Self::Json
        } else if content_type.starts_with("text/plain") {
            Self::Text
        } else {
            Self::Unsupported(content_type.to_string())
        }
    }
}

#[cfg(feature = "agents")]
pub mod agents_api;
#[cfg(feature = "assistants")]
pub mod assistants_api;
#[cfg(feature = "audio")]
pub mod audio_api;
#[cfg(feature = "administration")]
pub mod audit_logs_api;
#[cfg(feature = "batch")]
pub mod batch_api;
#[cfg(feature = "administration")]
pub mod certificates_api;
#[cfg(feature = "chat-completion")]
pub mod chat_api;
#[cfg(feature = "completions")]
pub mod completions_api;
#[cfg(feature = "responses")]
pub mod conversations_api;
#[cfg(feature = "administration")]
pub mod data_retention_api;
#[cfg(feature = "decisions")]
pub mod decisions_api;
#[cfg(any(
    feature = "administration",
    feature = "chatkit",
    feature = "container",
    feature = "content-provenance-checks",
    feature = "safety",
    feature = "webhook"
))]
pub mod default_api;
#[cfg(feature = "embedding")]
pub mod embeddings_api;
#[cfg(feature = "evals")]
pub mod evals_api;
#[cfg(feature = "file")]
pub mod files_api;
#[cfg(feature = "finetuning")]
pub mod fine_tuning_api;
#[cfg(feature = "administration")]
pub mod group_organization_role_assignments_api;
#[cfg(feature = "administration")]
pub mod group_users_api;
#[cfg(feature = "administration")]
pub mod groups_api;
#[cfg(feature = "administration")]
pub mod hosted_tools_api;
#[cfg(feature = "image")]
pub mod images_api;
#[cfg(feature = "administration")]
pub mod invites_api;
#[cfg(feature = "live")]
pub mod live_api;
#[cfg(feature = "model")]
pub mod models_api;
#[cfg(feature = "moderation")]
pub mod moderations_api;
#[cfg(feature = "administration")]
pub mod project_group_role_assignments_api;
#[cfg(feature = "administration")]
pub mod project_groups_api;
#[cfg(feature = "administration")]
pub mod project_user_role_assignments_api;
#[cfg(feature = "administration")]
pub mod projects_api;
#[cfg(feature = "realtime")]
pub mod realtime_api;
#[cfg(feature = "responses")]
pub mod responses_api;
#[cfg(feature = "administration")]
pub mod roles_api;
#[cfg(feature = "skill")]
pub mod skills_api;
#[cfg(feature = "administration")]
pub mod spend_alerts_api;
#[cfg(feature = "upload")]
pub mod uploads_api;
#[cfg(feature = "administration")]
pub mod usage_api;
#[cfg(feature = "administration")]
pub mod user_organization_role_assignments_api;
#[cfg(feature = "administration")]
pub mod users_api;
#[cfg(feature = "vaults")]
pub mod vaults_api;
#[cfg(feature = "vectorstore")]
pub mod vector_stores_api;
#[cfg(feature = "video")]
pub mod videos_api;

pub mod configuration;

// Internal helper module for multipart file handling
#[allow(dead_code)] // the API features enabled may not use every helper
pub(crate) mod multipart_helper;

pub mod sse;

#[cfg(feature = "open-ai-client-base-webhooks")]
pub mod open_ai_client_base_webhooks_api;
