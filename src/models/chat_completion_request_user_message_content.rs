use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionRequestUserMessageContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestUserMessageContent {
    TextContent(String),
    ArrayOfContentParts(Vec<models::ChatCompletionRequestUserMessageContentPart>),
}
