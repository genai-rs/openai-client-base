use crate::models;
use serde::{Deserialize, Serialize};

/// McpToolAllowedTools - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum McpToolAllowedTools {
    ArrayOfStrings(Vec<String>),
    Mcptoolfilter(models::McpToolFilter),
    Null,
}

impl Default for McpToolAllowedTools {
    fn default() -> Self {
        Self::ArrayOfStrings(Vec::new())
    }
}

impl std::fmt::Display for McpToolAllowedTools {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            McpToolAllowedTools::ArrayOfStrings(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            McpToolAllowedTools::Mcptoolfilter(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            McpToolAllowedTools::Null => write!(f, "null"),
        }
    }
}
