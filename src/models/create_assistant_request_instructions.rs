use crate::models;
use serde::{Deserialize, Serialize};

/// CreateAssistantRequestInstructions - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAssistantRequestInstructions {
    Text(String),
    Null,
}

impl Default for CreateAssistantRequestInstructions {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl CreateAssistantRequestInstructions {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for CreateAssistantRequestInstructions {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for CreateAssistantRequestInstructions {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for CreateAssistantRequestInstructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateAssistantRequestInstructions::Text(value) => write!(f, "{}", value),
            CreateAssistantRequestInstructions::Null => write!(f, "null"),
        }
    }
}

