use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionRequestDeveloperMessageContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestDeveloperMessageContent {
    TextContent(String),
    ArrayOfContentParts(Vec<models::ChatCompletionRequestMessageContentPartText>),
}








impl std::fmt::Display for ChatCompletionRequestDeveloperMessageContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatCompletionRequestDeveloperMessageContent::TextContent(value) => write!(f, "{}", value),
            ChatCompletionRequestDeveloperMessageContent::ArrayOfContentParts(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}

