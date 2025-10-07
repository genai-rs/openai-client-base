use crate::models;
use serde::{Deserialize, Serialize};

/// EasyInputMessageContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EasyInputMessageContent {
    TextInput(String),
    Inputmessagecontentlist(Vec<models::InputContent>),
}

impl Default for EasyInputMessageContent {
    fn default() -> Self {
        Self::TextInput(String::new())
    }
}

impl EasyInputMessageContent {
    pub fn new_text(text: String) -> Self {
        Self::TextInput(text)
    }
    pub fn new_inputmessagecontentlist(items: Vec<models::InputContent>) -> Self {
        Self::Inputmessagecontentlist(items)
    }
}

impl From<String> for EasyInputMessageContent {
    fn from(s: String) -> Self {
        Self::TextInput(s)
    }
}

impl From<&str> for EasyInputMessageContent {
    fn from(s: &str) -> Self {
        Self::TextInput(s.to_string())
    }
}
impl std::fmt::Display for EasyInputMessageContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EasyInputMessageContent::TextInput(value) => write!(f, "{}", value),
            EasyInputMessageContent::Inputmessagecontentlist(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
