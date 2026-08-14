use crate::models;
use serde::{Deserialize, Serialize};

/// BetaWebSearchApproximateLocation - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaWebSearchApproximateLocation {
    WebSearchApproximateLocation(models::ChatCompletionNamedToolChoice),
    Null,
}

impl std::fmt::Display for BetaWebSearchApproximateLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaWebSearchApproximateLocation::WebSearchApproximateLocation(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            BetaWebSearchApproximateLocation::Null => write!(f, "null"),
        }
    }
}
