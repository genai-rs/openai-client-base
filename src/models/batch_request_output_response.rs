use crate::models;
use serde::{Deserialize, Serialize};

/// BatchRequestOutputResponse - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BatchRequestOutputResponse {
    Null,
}

impl Default for BatchRequestOutputResponse {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for BatchRequestOutputResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BatchRequestOutputResponse::Null => write!(f, "null"),
        }
    }
}
