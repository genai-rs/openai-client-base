use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionRequestDeveloperMessageContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestDeveloperMessageContent {
    Textcontent(String),
    Arrayofcontentparts(Vec<models::ChatCompletionRequestMessageContentPartText>),
}

impl Default for ChatCompletionRequestDeveloperMessageContent {
    fn default() -> Self {
        Self::Textcontent(String::new())
    }
}

impl ChatCompletionRequestDeveloperMessageContent {
    pub fn new_text(text: String) -> Self {
        Self::Textcontent(text)
    }
    pub fn new_arrayofcontentparts(items: Vec<models::ChatCompletionRequestMessageContentPartText>) -> Self {
        Self::Arrayofcontentparts(items)
    }
}

impl From<String> for ChatCompletionRequestDeveloperMessageContent {
    fn from(s: String) -> Self {
        Self::Textcontent(s)
    }
}

impl From<&str> for ChatCompletionRequestDeveloperMessageContent {
    fn from(s: &str) -> Self {
        Self::Textcontent(s.to_string())
    }
}
