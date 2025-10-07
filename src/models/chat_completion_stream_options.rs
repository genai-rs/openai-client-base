use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionStreamOptions - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionStreamOptions {
    Null,
}

impl Default for ChatCompletionStreamOptions {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for ChatCompletionStreamOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatCompletionStreamOptions::Null => write!(f, "null"),
        }
    }
}
