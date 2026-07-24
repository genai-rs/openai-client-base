use crate::models;
use serde::{Deserialize, Serialize};

/// BetaResponseStreamOptions - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaResponseStreamOptions {
    Null,
}

impl Default for BetaResponseStreamOptions {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for BetaResponseStreamOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaResponseStreamOptions::Null => write!(f, "null"),
        }
    }
}
