use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionRequestSystemMessageContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestSystemMessageContent {
    Text(String),
    Array(Vec<models::ChatCompletionRequestMessageContentPartText>),
}

impl Default for ChatCompletionRequestSystemMessageContent {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl ChatCompletionRequestSystemMessageContent {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_array(array: Vec<models::ChatCompletionRequestMessageContentPartText>) -> Self {
        Self::Array(array)
    }
}

impl From<String> for ChatCompletionRequestSystemMessageContent {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for ChatCompletionRequestSystemMessageContent {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
