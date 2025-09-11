use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionRequestToolMessageContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestToolMessageContent {
    Text(String),
    Array(Vec<models::ChatCompletionRequestMessageContentPartText>),
}

impl Default for ChatCompletionRequestToolMessageContent {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl ChatCompletionRequestToolMessageContent {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_array(array: Vec<models::ChatCompletionRequestMessageContentPartText>) -> Self {
        Self::Array(array)
    }
}

impl From<String> for ChatCompletionRequestToolMessageContent {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for ChatCompletionRequestToolMessageContent {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
