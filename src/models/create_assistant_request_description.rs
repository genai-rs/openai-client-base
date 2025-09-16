use crate::models;
use serde::{Deserialize, Serialize};

/// CreateAssistantRequestDescription - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAssistantRequestDescription {
    Text(String),
    Null,
}

impl Default for CreateAssistantRequestDescription {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl CreateAssistantRequestDescription {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for CreateAssistantRequestDescription {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for CreateAssistantRequestDescription {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for CreateAssistantRequestDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateAssistantRequestDescription::Text(value) => write!(f, "{}", value),
            CreateAssistantRequestDescription::Null => write!(f, "null"),
        }
    }
}
