use crate::models;
use serde::{Deserialize, Serialize};

/// BetaResponseError - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaResponseError {
    Object0(models::ChatCompletionNamedToolChoice),
    Null,
}

impl std::fmt::Display for BetaResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaResponseError::Object0(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            BetaResponseError::Null => write!(f, "null"),
        }
    }
}
