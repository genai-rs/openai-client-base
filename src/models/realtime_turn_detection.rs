use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeTurnDetection - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeTurnDetection {
    ServerVad(models::ChatCompletionNamedToolChoice),
    SemanticVad(models::ChatCompletionNamedToolChoice),
    Null,
}

impl std::fmt::Display for RealtimeTurnDetection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RealtimeTurnDetection::ServerVad(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            RealtimeTurnDetection::SemanticVad(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            RealtimeTurnDetection::Null => write!(f, "null"),
        }
    }
}

