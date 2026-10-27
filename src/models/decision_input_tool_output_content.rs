use serde::{Deserialize, Serialize};

/// DecisionInputToolOutputContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DecisionInputToolOutputContent {
    Text(String),
    Decisioninputtooloutputcontentparts(Vec<serde_json::Value>),
}

impl Default for DecisionInputToolOutputContent {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl DecisionInputToolOutputContent {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_decisioninputtooloutputcontentparts(items: Vec<serde_json::Value>) -> Self {
        Self::Decisioninputtooloutputcontentparts(items)
    }
}

impl From<String> for DecisionInputToolOutputContent {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for DecisionInputToolOutputContent {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for DecisionInputToolOutputContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecisionInputToolOutputContent::Text(value) => write!(f, "{}", value),
            DecisionInputToolOutputContent::Decisioninputtooloutputcontentparts(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
