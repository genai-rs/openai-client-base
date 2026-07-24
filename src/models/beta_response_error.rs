use crate::models;
use serde::{Deserialize, Serialize};

/// BetaResponseError - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaResponseError {
    Null,
}

impl Default for BetaResponseError {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for BetaResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaResponseError::Null => write!(f, "null"),
        }
    }
}
