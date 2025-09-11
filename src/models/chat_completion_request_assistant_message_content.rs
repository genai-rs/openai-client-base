use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionRequestAssistantMessageContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestAssistantMessageContent {
    TextContent(String),
    ArrayOfContentParts(Vec<models::ChatCompletionRequestAssistantMessageContentPart>),
}

impl Default for ChatCompletionRequestAssistantMessageContent {
    fn default() -> Self {
        Self::TextContent(String::new())
    }
}

impl ChatCompletionRequestAssistantMessageContent {
    pub fn new_text(text: String) -> Self {
        Self::TextContent(text)
    }
    pub fn new_arrayofcontentparts(items: Vec<models::ChatCompletionRequestAssistantMessageContentPart>) -> Self {
        Self::ArrayOfContentParts(items)
    }
}

impl From<String> for ChatCompletionRequestAssistantMessageContent {
    fn from(s: String) -> Self {
        Self::TextContent(s)
    }
}

impl From<&str> for ChatCompletionRequestAssistantMessageContent {
    fn from(s: &str) -> Self {
        Self::TextContent(s.to_string())
    }
}
