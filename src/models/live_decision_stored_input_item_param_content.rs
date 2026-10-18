use crate::models;
use serde::{Deserialize, Serialize};

/// LiveDecisionStoredInputItemParamContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LiveDecisionStoredInputItemParamContent {
    Text(String),
    Arrayofitems(Vec<serde_json::Value>),
}

impl Default for LiveDecisionStoredInputItemParamContent {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl LiveDecisionStoredInputItemParamContent {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_arrayofitems(items: Vec<serde_json::Value>) -> Self {
        Self::Arrayofitems(items)
    }
}

impl From<String> for LiveDecisionStoredInputItemParamContent {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for LiveDecisionStoredInputItemParamContent {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for LiveDecisionStoredInputItemParamContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiveDecisionStoredInputItemParamContent::Text(value) => write!(f, "{}", value),
            LiveDecisionStoredInputItemParamContent::Arrayofitems(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
