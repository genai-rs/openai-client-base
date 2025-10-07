use crate::models;
use serde::{Deserialize, Serialize};

/// Prompt - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Prompt {
    Null,
}

impl Default for Prompt {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for Prompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Prompt::Null => write!(f, "null"),
        }
    }
}

