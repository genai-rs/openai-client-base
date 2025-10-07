use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionRequestUserMessageContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestUserMessageContent {
    TextContent(String),
    ArrayOfContentParts(Vec<models::ChatCompletionRequestUserMessageContentPart>),
}

impl std::fmt::Display for ChatCompletionRequestUserMessageContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatCompletionRequestUserMessageContent::TextContent(value) => write!(f, "{}", value),
            ChatCompletionRequestUserMessageContent::ArrayOfContentParts(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
