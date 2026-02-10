use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionRequestUserMessageContentPart - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestUserMessageContentPart {
    Chatcompletionrequestmessagecontentparttext(
        models::ChatCompletionRequestMessageContentPartText,
    ),
    Chatcompletionrequestmessagecontentpartimage(
        models::ChatCompletionRequestMessageContentPartImage,
    ),
    Chatcompletionrequestmessagecontentpartaudio(
        models::ChatCompletionRequestMessageContentPartAudio,
    ),
    Chatcompletionrequestmessagecontentpartfile(
        models::ChatCompletionRequestMessageContentPartFile,
    ),
}

impl std::fmt::Display for ChatCompletionRequestUserMessageContentPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatCompletionRequestUserMessageContentPart::Chatcompletionrequestmessagecontentparttext(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            ChatCompletionRequestUserMessageContentPart::Chatcompletionrequestmessagecontentpartimage(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            ChatCompletionRequestUserMessageContentPart::Chatcompletionrequestmessagecontentpartaudio(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            ChatCompletionRequestUserMessageContentPart::Chatcompletionrequestmessagecontentpartfile(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}
