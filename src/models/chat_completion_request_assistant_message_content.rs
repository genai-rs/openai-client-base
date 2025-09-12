use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionRequestAssistantMessageContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestAssistantMessageContent {
    TextContent(String),
    ArrayOfContentParts(Vec<models::ChatCompletionRequestAssistantMessageContentPart>),
}
