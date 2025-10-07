use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionRequestAssistantMessageContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestAssistantMessageContent {
    TextContent(String),
    ArrayOfContentParts(Vec<models::ChatCompletionRequestAssistantMessageContentPart>),
    Null,
}








impl std::fmt::Display for ChatCompletionRequestAssistantMessageContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatCompletionRequestAssistantMessageContent::TextContent(value) => write!(f, "{}", value),
            ChatCompletionRequestAssistantMessageContent::ArrayOfContentParts(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            ChatCompletionRequestAssistantMessageContent::Null => write!(f, "null"),
        }
    }
}

