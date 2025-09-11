use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionRequestUserMessageContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestUserMessageContent {
    Textcontent(String),
    Arrayofcontentparts(Vec<models::ChatCompletionRequestUserMessageContentPart>),
}

impl Default for ChatCompletionRequestUserMessageContent {
    fn default() -> Self {
        Self::Textcontent(String::new())
    }
}

impl ChatCompletionRequestUserMessageContent {
    pub fn new_text(text: String) -> Self {
        Self::Textcontent(text)
    }
    pub fn new_arrayofcontentparts(items: Vec<models::ChatCompletionRequestUserMessageContentPart>) -> Self {
        Self::Arrayofcontentparts(items)
    }
}

impl From<String> for ChatCompletionRequestUserMessageContent {
    fn from(s: String) -> Self {
        Self::Textcontent(s)
    }
}

impl From<&str> for ChatCompletionRequestUserMessageContent {
    fn from(s: &str) -> Self {
        Self::Textcontent(s.to_string())
    }
}
