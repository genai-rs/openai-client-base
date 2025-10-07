use crate::models;
use serde::{Deserialize, Serialize};

/// BatchRequestOutputError - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BatchRequestOutputError {
    Null,
}

impl Default for BatchRequestOutputError {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for BatchRequestOutputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BatchRequestOutputError::Null => write!(f, "null"),
        }
    }
}

