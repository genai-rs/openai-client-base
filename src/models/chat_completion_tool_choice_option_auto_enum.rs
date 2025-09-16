use serde::{Deserialize, Serialize};

/// ChatCompletionToolChoiceOptionAutoEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatCompletionToolChoiceOptionAutoEnum {
    None,
    Auto,
    Required,
}

impl Default for ChatCompletionToolChoiceOptionAutoEnum {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for ChatCompletionToolChoiceOptionAutoEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ChatCompletionToolChoiceOptionAutoEnum::None => "none",
            ChatCompletionToolChoiceOptionAutoEnum::Auto => "auto",
            ChatCompletionToolChoiceOptionAutoEnum::Required => "required",
        };
        write!(f, "{}", value)
    }
}
