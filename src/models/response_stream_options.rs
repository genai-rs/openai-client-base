use crate::models;
use serde::{Deserialize, Serialize};

/// ResponseStreamOptions - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseStreamOptions {
    Null,
}

impl Default for ResponseStreamOptions {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for ResponseStreamOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseStreamOptions::Null => write!(f, "null"),
        }
    }
}
