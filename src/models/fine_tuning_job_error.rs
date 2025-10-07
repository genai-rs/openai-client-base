use crate::models;
use serde::{Deserialize, Serialize};

/// FineTuningJobError - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuningJobError {
    Null,
}

impl Default for FineTuningJobError {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for FineTuningJobError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FineTuningJobError::Null => write!(f, "null"),
        }
    }
}

