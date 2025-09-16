use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionRequestAssistantMessageAudio - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestAssistantMessageAudio {
    Null,
}

impl Default for ChatCompletionRequestAssistantMessageAudio {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for ChatCompletionRequestAssistantMessageAudio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatCompletionRequestAssistantMessageAudio::Null => write!(f, "null"),
        }
    }
}
