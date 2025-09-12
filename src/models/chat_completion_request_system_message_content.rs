use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionRequestSystemMessageContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestSystemMessageContent {
    TextContent(String),
    ArrayOfContentParts(Vec<models::ChatCompletionRequestMessageContentPartText>),
}

impl Default for ChatCompletionRequestSystemMessageContent {
    fn default() -> Self {
        Self::TextContent(String::new())
    }
}

impl ChatCompletionRequestSystemMessageContent {
    pub fn new_text(text: String) -> Self {
        Self::TextContent(text)
    }
    pub fn new_arrayofcontentparts(
        items: Vec<models::ChatCompletionRequestMessageContentPartText>,
    ) -> Self {
        Self::ArrayOfContentParts(items)
    }
}

impl From<String> for ChatCompletionRequestSystemMessageContent {
    fn from(s: String) -> Self {
        Self::TextContent(s)
    }
}

impl From<&str> for ChatCompletionRequestSystemMessageContent {
    fn from(s: &str) -> Self {
        Self::TextContent(s.to_string())
    }
}
