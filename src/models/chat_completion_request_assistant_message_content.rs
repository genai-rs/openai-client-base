use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionRequestAssistantMessageContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestAssistantMessageContent {
    Textcontent(String),
    Arrayofcontentparts(Vec<models::ChatCompletionRequestAssistantMessageContentPart>),
}

impl Default for ChatCompletionRequestAssistantMessageContent {
    fn default() -> Self {
        Self::Textcontent(String::new())
    }
}

impl ChatCompletionRequestAssistantMessageContent {
    pub fn new_text(text: String) -> Self {
        Self::Textcontent(text)
    }
    pub fn new_arrayofcontentparts(items: Vec<models::ChatCompletionRequestAssistantMessageContentPart>) -> Self {
        Self::Arrayofcontentparts(items)
    }
}

impl From<String> for ChatCompletionRequestAssistantMessageContent {
    fn from(s: String) -> Self {
        Self::Textcontent(s)
    }
}

impl From<&str> for ChatCompletionRequestAssistantMessageContent {
    fn from(s: &str) -> Self {
        Self::Textcontent(s.to_string())
    }
}
