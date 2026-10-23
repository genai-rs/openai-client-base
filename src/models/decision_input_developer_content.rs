use crate::models;
use serde::{Deserialize, Serialize};

/// DecisionInputDeveloperContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DecisionInputDeveloperContent {
    Text(String),
    Decisioninputdevelopercontentparts(Vec<models::DecisionInputText>),
}

impl Default for DecisionInputDeveloperContent {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl DecisionInputDeveloperContent {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_decisioninputdevelopercontentparts(items: Vec<models::DecisionInputText>) -> Self {
        Self::Decisioninputdevelopercontentparts(items)
    }
}

impl From<String> for DecisionInputDeveloperContent {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for DecisionInputDeveloperContent {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for DecisionInputDeveloperContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecisionInputDeveloperContent::Text(value) => write!(f, "{}", value),
            DecisionInputDeveloperContent::Decisioninputdevelopercontentparts(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
