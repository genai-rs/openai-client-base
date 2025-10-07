use crate::models;
use serde::{Deserialize, Serialize};

/// ResponseError - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseError {
    Null,
}

impl Default for ResponseError {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseError::Null => write!(f, "null"),
        }
    }
}

