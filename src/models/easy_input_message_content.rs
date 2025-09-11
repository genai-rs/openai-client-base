use crate::models;
use serde::{Deserialize, Serialize};

/// EasyInputMessageContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EasyInputMessageContent {
    Textinput(String),
    InputMessageContentList(models::InputMessageContentList),
}

impl Default for EasyInputMessageContent {
    fn default() -> Self {
        Self::Textinput(String::new())
    }
}

impl EasyInputMessageContent {
    pub fn new_text(text: String) -> Self {
        Self::Textinput(text)
    }
}

impl From<String> for EasyInputMessageContent {
    fn from(s: String) -> Self {
        Self::Textinput(s)
    }
}

impl From<&str> for EasyInputMessageContent {
    fn from(s: &str) -> Self {
        Self::Textinput(s.to_string())
    }
}
