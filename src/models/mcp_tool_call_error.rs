use crate::models;
use serde::{Deserialize, Serialize};

/// McpToolCallError - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum McpToolCallError {
    Mcptoolcallerror(Box<models::McpToolCallError>),
    Null,
}

impl std::fmt::Display for McpToolCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            McpToolCallError::Mcptoolcallerror(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            McpToolCallError::Null => write!(f, "null"),
        }
    }
}
