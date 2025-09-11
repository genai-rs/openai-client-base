use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionRequestAssistantMessageContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestAssistantMessageContent {
    Text(String),
    Array(Vec<models::ChatCompletionRequestAssistantMessageContentPart>),
}

impl Default for ChatCompletionRequestAssistantMessageContent {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl ChatCompletionRequestAssistantMessageContent {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_array(array: Vec<models::ChatCompletionRequestAssistantMessageContentPart>) -> Self {
        Self::Array(array)
    }
}

impl From<String> for ChatCompletionRequestAssistantMessageContent {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for ChatCompletionRequestAssistantMessageContent {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
