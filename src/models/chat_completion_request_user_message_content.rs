use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionRequestUserMessageContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestUserMessageContent {
    Text(String),
    Array(Vec<models::ChatCompletionRequestUserMessageContentPart>),
}

impl Default for ChatCompletionRequestUserMessageContent {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl ChatCompletionRequestUserMessageContent {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_array(array: Vec<models::ChatCompletionRequestUserMessageContentPart>) -> Self {
        Self::Array(array)
    }
}

impl From<String> for ChatCompletionRequestUserMessageContent {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for ChatCompletionRequestUserMessageContent {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
