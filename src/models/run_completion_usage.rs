use crate::models;
use serde::{Deserialize, Serialize};

/// RunCompletionUsage - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RunCompletionUsage {
    Null,
}

impl Default for RunCompletionUsage {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for RunCompletionUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunCompletionUsage::Null => write!(f, "null"),
        }
    }
}

