use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionRequestSystemMessageContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestSystemMessageContent {
    Textcontent(String),
    Arrayofcontentparts(Vec<models::ChatCompletionRequestSystemMessageContentPart>),
}

impl Default for ChatCompletionRequestSystemMessageContent {
    fn default() -> Self {
        Self::Textcontent(String::new())
    }
}

impl ChatCompletionRequestSystemMessageContent {
    pub fn new_text(text: String) -> Self {
        Self::Textcontent(text)
    }
    pub fn new_arrayofcontentparts(items: Vec<models::ChatCompletionRequestSystemMessageContentPart>) -> Self {
        Self::Arrayofcontentparts(items)
    }
}

impl From<String> for ChatCompletionRequestSystemMessageContent {
    fn from(s: String) -> Self {
        Self::Textcontent(s)
    }
}

impl From<&str> for ChatCompletionRequestSystemMessageContent {
    fn from(s: &str) -> Self {
        Self::Textcontent(s.to_string())
    }
}
