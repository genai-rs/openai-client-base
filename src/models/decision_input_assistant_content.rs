use crate::models;
use serde::{Deserialize, Serialize};

/// DecisionInputAssistantContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DecisionInputAssistantContent {
    Text(String),
    Decisioninputassistantcontentparts(Vec<serde_json::Value>),
}

impl Default for DecisionInputAssistantContent {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl DecisionInputAssistantContent {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_decisioninputassistantcontentparts(items: Vec<serde_json::Value>) -> Self {
        Self::Decisioninputassistantcontentparts(items)
    }
}

impl From<String> for DecisionInputAssistantContent {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for DecisionInputAssistantContent {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for DecisionInputAssistantContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecisionInputAssistantContent::Text(value) => write!(f, "{}", value),
            DecisionInputAssistantContent::Decisioninputassistantcontentparts(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
