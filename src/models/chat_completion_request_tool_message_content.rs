use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionRequestToolMessageContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestToolMessageContent {
    TextContent(String),
    ArrayOfContentParts(Vec<models::ChatCompletionRequestMessageContentPartText>),
}

impl std::fmt::Display for ChatCompletionRequestToolMessageContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatCompletionRequestToolMessageContent::TextContent(value) => write!(f, "{}", value),
            ChatCompletionRequestToolMessageContent::ArrayOfContentParts(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
