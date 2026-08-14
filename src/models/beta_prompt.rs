use crate::models;
use serde::{Deserialize, Serialize};

/// BetaPrompt - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaPrompt {
    Null,
}

impl Default for BetaPrompt {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for BetaPrompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaPrompt::Null => write!(f, "null"),
        }
    }
}
