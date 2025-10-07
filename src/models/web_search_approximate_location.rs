use crate::models;
use serde::{Deserialize, Serialize};

/// WebSearchApproximateLocation - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebSearchApproximateLocation {
    WebSearchApproximateLocation(models::ChatCompletionNamedToolChoice),
    Null,
}

impl std::fmt::Display for WebSearchApproximateLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebSearchApproximateLocation::WebSearchApproximateLocation(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            WebSearchApproximateLocation::Null => write!(f, "null"),
        }
    }
}
