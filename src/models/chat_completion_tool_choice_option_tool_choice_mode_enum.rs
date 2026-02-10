use serde::{Deserialize, Serialize};

/// ChatCompletionToolChoiceOptionToolChoiceModeEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatCompletionToolChoiceOptionToolChoiceModeEnum {
    None,
    Auto,
    Required,
}

impl Default for ChatCompletionToolChoiceOptionToolChoiceModeEnum {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for ChatCompletionToolChoiceOptionToolChoiceModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ChatCompletionToolChoiceOptionToolChoiceModeEnum::None => "none",
            ChatCompletionToolChoiceOptionToolChoiceModeEnum::Auto => "auto",
            ChatCompletionToolChoiceOptionToolChoiceModeEnum::Required => "required",
        };
        write!(f, "{}", value)
    }
}
