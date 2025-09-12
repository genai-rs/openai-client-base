use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionRequestAssistantMessageContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ChatCompletionRequestAssistantMessageContent {
    #[serde(rename = "text")]
    ChatCompletionRequestMessageContentPartText(Box<models::ChatCompletionRequestMessageContentPartText>),
    #[serde(rename = "refusal")]
    ChatCompletionRequestMessageContentPartRefusal(Box<models::ChatCompletionRequestMessageContentPartRefusal>),
}

// Helper impls removed: enum is tagged and does not expose TextContent/ArrayOfContentParts variants.
