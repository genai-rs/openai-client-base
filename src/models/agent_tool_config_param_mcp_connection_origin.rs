use crate::models;
use serde::{Deserialize, Serialize};

/// AgentToolConfigParamMcpConnectionOrigin - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AgentToolConfigParamMcpConnectionOrigin {
    Mcpconnectionoriginparam(models::McpConnectionOriginParam),
    Null,
}

impl std::fmt::Display for AgentToolConfigParamMcpConnectionOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentToolConfigParamMcpConnectionOrigin::Mcpconnectionoriginparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            AgentToolConfigParamMcpConnectionOrigin::Null => write!(f, "null"),
        }
    }
}
