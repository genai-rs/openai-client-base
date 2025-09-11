use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionToolChoiceOption - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionToolChoiceOption {
    Auto(ChatCompletionToolChoiceOptionAuto),
    Named(models::ChatCompletionNamedToolChoice),
}

impl Default for ChatCompletionToolChoiceOption {
    fn default() -> Self {
        Self::Auto(Default::default())
    }
}

/// ChatCompletionToolChoiceOptionAuto - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatCompletionToolChoiceOptionAuto {
    None,
    Auto,
    Required,
}

impl Default for ChatCompletionToolChoiceOptionAuto {
    fn default() -> Self {
        Self::None
    }
}
