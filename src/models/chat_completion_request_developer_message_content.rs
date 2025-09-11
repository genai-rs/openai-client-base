use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionRequestDeveloperMessageContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestDeveloperMessageContent {
    TextContent(String),
    ArrayOfContentParts(Vec<models::ChatCompletionRequestMessageContentPartText>),
}

impl Default for ChatCompletionRequestDeveloperMessageContent {
    fn default() -> Self {
        Self::TextContent(String::new())
    }
}

impl ChatCompletionRequestDeveloperMessageContent {
    pub fn new_text(text: String) -> Self {
        Self::TextContent(text)
    }
    pub fn new_arrayofcontentparts(
        items: Vec<models::ChatCompletionRequestMessageContentPartText>,
    ) -> Self {
        Self::ArrayOfContentParts(items)
    }
}

impl From<String> for ChatCompletionRequestDeveloperMessageContent {
    fn from(s: String) -> Self {
        Self::TextContent(s)
    }
}

impl From<&str> for ChatCompletionRequestDeveloperMessageContent {
    fn from(s: &str) -> Self {
        Self::TextContent(s.to_string())
    }
}
