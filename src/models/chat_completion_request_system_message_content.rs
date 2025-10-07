use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionRequestSystemMessageContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestSystemMessageContent {
    TextContent(String),
    ArrayOfContentParts(Vec<models::ChatCompletionRequestMessageContentPartText>),
}








impl std::fmt::Display for ChatCompletionRequestSystemMessageContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatCompletionRequestSystemMessageContent::TextContent(value) => write!(f, "{}", value),
            ChatCompletionRequestSystemMessageContent::ArrayOfContentParts(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}

