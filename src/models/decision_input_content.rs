use crate::models;
use serde::{Deserialize, Serialize};

/// DecisionInputContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DecisionInputContent {
    Text(String),
    Decisioninputcontentparts(Vec<serde_json::Value>),
}

impl Default for DecisionInputContent {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl DecisionInputContent {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_decisioninputcontentparts(items: Vec<serde_json::Value>) -> Self {
        Self::Decisioninputcontentparts(items)
    }
}

impl From<String> for DecisionInputContent {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for DecisionInputContent {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for DecisionInputContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecisionInputContent::Text(value) => write!(f, "{}", value),
            DecisionInputContent::Decisioninputcontentparts(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
