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

// Helper impls removed: enum is tagged and does not expose TextContent/ArrayOfContentParts variants.
