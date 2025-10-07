use crate::models;
use serde::{Deserialize, Serialize};

/// CreateAssistantRequestName - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAssistantRequestName {
    Text(String),
    Null,
}

impl Default for CreateAssistantRequestName {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl CreateAssistantRequestName {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for CreateAssistantRequestName {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for CreateAssistantRequestName {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for CreateAssistantRequestName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateAssistantRequestName::Text(value) => write!(f, "{}", value),
            CreateAssistantRequestName::Null => write!(f, "null"),
        }
    }
}
