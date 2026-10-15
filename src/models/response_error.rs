use crate::models;
use serde::{Deserialize, Serialize};

/// ResponseError - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseError {
    Object0(models::ChatCompletionNamedToolChoice),
    Null,
}

impl std::fmt::Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseError::Object0(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            ResponseError::Null => write!(f, "null"),
        }
    }
}
