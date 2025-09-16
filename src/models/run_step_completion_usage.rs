use crate::models;
use serde::{Deserialize, Serialize};

/// RunStepCompletionUsage - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RunStepCompletionUsage {
    Null,
}

impl Default for RunStepCompletionUsage {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for RunStepCompletionUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunStepCompletionUsage::Null => write!(f, "null"),
        }
    }
}
