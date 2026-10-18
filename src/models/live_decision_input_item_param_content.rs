use crate::models;
use serde::{Deserialize, Serialize};

/// LiveDecisionInputItemParamContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LiveDecisionInputItemParamContent {
    Text(String),
    Arrayofitems(Vec<serde_json::Value>),
}

impl Default for LiveDecisionInputItemParamContent {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl LiveDecisionInputItemParamContent {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_arrayofitems(items: Vec<serde_json::Value>) -> Self {
        Self::Arrayofitems(items)
    }
}

impl From<String> for LiveDecisionInputItemParamContent {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for LiveDecisionInputItemParamContent {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for LiveDecisionInputItemParamContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiveDecisionInputItemParamContent::Text(value) => write!(f, "{}", value),
            LiveDecisionInputItemParamContent::Arrayofitems(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
