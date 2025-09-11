use crate::models;
use serde::{Deserialize, Serialize};

/// McpToolAllowedTools - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum McpToolAllowedTools {
    ArrayOfStrings(Vec<String>),
    Mcptoolfilter(models::McpToolFilter),
}

impl Default for McpToolAllowedTools {
    fn default() -> Self {
        Self::ArrayOfStrings(Vec::new())
    }
}

