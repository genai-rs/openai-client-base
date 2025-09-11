use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionRequestToolMessageContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestToolMessageContent {
    Textcontent(String),
    Arrayofcontentparts(Vec<models::ChatCompletionRequestToolMessageContentPart>),
}

impl Default for ChatCompletionRequestToolMessageContent {
    fn default() -> Self {
        Self::Textcontent(String::new())
    }
}

impl ChatCompletionRequestToolMessageContent {
    pub fn new_text(text: String) -> Self {
        Self::Textcontent(text)
    }
    pub fn new_arrayofcontentparts(items: Vec<models::ChatCompletionRequestToolMessageContentPart>) -> Self {
        Self::Arrayofcontentparts(items)
    }
}

impl From<String> for ChatCompletionRequestToolMessageContent {
    fn from(s: String) -> Self {
        Self::Textcontent(s)
    }
}

impl From<&str> for ChatCompletionRequestToolMessageContent {
    fn from(s: &str) -> Self {
        Self::Textcontent(s.to_string())
    }
}
