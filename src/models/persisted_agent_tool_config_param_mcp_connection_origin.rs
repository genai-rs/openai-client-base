use crate::models;
use serde::{Deserialize, Serialize};

/// PersistedAgentToolConfigParamMcpConnectionOrigin - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PersistedAgentToolConfigParamMcpConnectionOrigin {
    Mcpconnectionoriginparam(models::McpConnectionOriginParam),
    Null,
}

impl std::fmt::Display for PersistedAgentToolConfigParamMcpConnectionOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PersistedAgentToolConfigParamMcpConnectionOrigin::Mcpconnectionoriginparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            PersistedAgentToolConfigParamMcpConnectionOrigin::Null => write!(f, "null"),
        }
    }
}
