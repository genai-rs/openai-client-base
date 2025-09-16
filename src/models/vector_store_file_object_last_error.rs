use crate::models;
use serde::{Deserialize, Serialize};

/// VectorStoreFileObjectLastError - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VectorStoreFileObjectLastError {
    Null,
}

impl Default for VectorStoreFileObjectLastError {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for VectorStoreFileObjectLastError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VectorStoreFileObjectLastError::Null => write!(f, "null"),
        }
    }
}
