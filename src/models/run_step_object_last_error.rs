use crate::models;
use serde::{Deserialize, Serialize};

/// RunStepObjectLastError - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RunStepObjectLastError {
    Null,
}

impl Default for RunStepObjectLastError {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for RunStepObjectLastError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunStepObjectLastError::Null => write!(f, "null"),
        }
    }
}
