use crate::models;
use serde::{Deserialize, Serialize};

/// BetaEasyInputMessageContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaEasyInputMessageContent {
    TextInput(String),
    Betainputmessagecontentlist(Vec<models::BetaInputContent>),
}

impl Default for BetaEasyInputMessageContent {
    fn default() -> Self {
        Self::TextInput(String::new())
    }
}

impl BetaEasyInputMessageContent {
    pub fn new_text(text: String) -> Self {
        Self::TextInput(text)
    }
    pub fn new_betainputmessagecontentlist(items: Vec<models::BetaInputContent>) -> Self {
        Self::Betainputmessagecontentlist(items)
    }
}

impl From<String> for BetaEasyInputMessageContent {
    fn from(s: String) -> Self {
        Self::TextInput(s)
    }
}

impl From<&str> for BetaEasyInputMessageContent {
    fn from(s: &str) -> Self {
        Self::TextInput(s.to_string())
    }
}
impl std::fmt::Display for BetaEasyInputMessageContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaEasyInputMessageContent::TextInput(value) => write!(f, "{}", value),
            BetaEasyInputMessageContent::Betainputmessagecontentlist(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
