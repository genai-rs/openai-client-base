use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionRequestUserMessageContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ChatCompletionRequestUserMessageContent {
    #[serde(rename = "text")]
    ChatCompletionRequestMessageContentPartText(Box<models::ChatCompletionRequestMessageContentPartText>),
    #[serde(rename = "image_url")]
    ChatCompletionRequestMessageContentPartImage(Box<models::ChatCompletionRequestMessageContentPartImage>),
    #[serde(rename = "input_audio")]
    ChatCompletionRequestMessageContentPartAudio(Box<models::ChatCompletionRequestMessageContentPartAudio>),
    #[serde(rename = "file")]
    ChatCompletionRequestMessageContentPartFile(Box<models::ChatCompletionRequestMessageContentPartFile>),
}

impl Default for ChatCompletionRequestUserMessageContent {
    fn default() -> Self {
        Self::TextContent(String::new())
    }
}

impl ChatCompletionRequestUserMessageContent {
    pub fn new_text(text: String) -> Self {
        Self::TextContent(text)
    }
    pub fn new_arrayofcontentparts(
        items: Vec<models::ChatCompletionRequestUserMessageContentPart>,
    ) -> Self {
        Self::ArrayOfContentParts(items)
    }
}

impl From<String> for ChatCompletionRequestUserMessageContent {
    fn from(s: String) -> Self {
        Self::TextContent(s)
    }
}

impl From<&str> for ChatCompletionRequestUserMessageContent {
    fn from(s: &str) -> Self {
        Self::TextContent(s.to_string())
    }
}
