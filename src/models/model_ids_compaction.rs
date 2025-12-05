use crate::models;
use serde::{Deserialize, Serialize};

/// ModelIdsCompaction - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModelIdsCompaction {
    Text(String),
    Null,
}

impl std::fmt::Display for ModelIdsCompaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelIdsCompaction::Text(value) => write!(f, "{}", value),
            ModelIdsCompaction::Null => write!(f, "null"),
        }
    }
}
