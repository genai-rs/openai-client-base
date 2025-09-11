use serde::{Deserialize, Serialize};

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
