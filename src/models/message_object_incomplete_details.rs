use crate::models;
use serde::{Deserialize, Serialize};

/// MessageObjectIncompleteDetails - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageObjectIncompleteDetails {
    Null,
}

impl Default for MessageObjectIncompleteDetails {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for MessageObjectIncompleteDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageObjectIncompleteDetails::Null => write!(f, "null"),
        }
    }
}

