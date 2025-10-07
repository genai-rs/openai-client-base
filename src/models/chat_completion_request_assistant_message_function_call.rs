use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionRequestAssistantMessageFunctionCall - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestAssistantMessageFunctionCall {
    Null,
}

impl Default for ChatCompletionRequestAssistantMessageFunctionCall {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for ChatCompletionRequestAssistantMessageFunctionCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatCompletionRequestAssistantMessageFunctionCall::Null => write!(f, "null"),
        }
    }
}

