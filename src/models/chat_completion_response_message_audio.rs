use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionResponseMessageAudio - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionResponseMessageAudio {
    Null,
}

impl Default for ChatCompletionResponseMessageAudio {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for ChatCompletionResponseMessageAudio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatCompletionResponseMessageAudio::Null => write!(f, "null"),
        }
    }
}
