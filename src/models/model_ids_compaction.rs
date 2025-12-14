use crate::models;
use serde::{Deserialize, Serialize};

/// ModelIdsCompaction - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModelIdsCompaction {
    Text(String),
    TextString(String),
    Null,
}

impl Default for ModelIdsCompaction {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl ModelIdsCompaction {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for ModelIdsCompaction {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for ModelIdsCompaction {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for ModelIdsCompaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelIdsCompaction::Text(value) => write!(f, "{}", value),
            ModelIdsCompaction::TextString(value) => write!(f, "{}", value),
            ModelIdsCompaction::Null => write!(f, "null"),
        }
    }
}
