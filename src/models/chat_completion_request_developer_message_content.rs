use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionRequestDeveloperMessageContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestDeveloperMessageContent {
    Text(String),
    Array(Vec<models::ChatCompletionRequestMessageContentPartText>),
}

impl Default for ChatCompletionRequestDeveloperMessageContent {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl ChatCompletionRequestDeveloperMessageContent {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_array(array: Vec<models::ChatCompletionRequestMessageContentPartText>) -> Self {
        Self::Array(array)
    }
}

impl From<String> for ChatCompletionRequestDeveloperMessageContent {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for ChatCompletionRequestDeveloperMessageContent {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
