use crate::models;
use serde::{Deserialize, Serialize};

/// MCPToolAllowedTools - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MCPToolAllowedTools {
    ArrayOfStrings(Vec<String>),
    Mcptoolfilter(models::McpToolFilter),
}

impl Default for MCPToolAllowedTools {
    fn default() -> Self {
        Self::ArrayOfStrings(Vec::new())
    }
}

