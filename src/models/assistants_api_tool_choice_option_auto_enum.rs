use serde::{Deserialize, Serialize};

/// AssistantsApiToolChoiceOptionAutoEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssistantsApiToolChoiceOptionAutoEnum {
    None,
    Auto,
    Required,
}

impl Default for AssistantsApiToolChoiceOptionAutoEnum {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for AssistantsApiToolChoiceOptionAutoEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            AssistantsApiToolChoiceOptionAutoEnum::None => "none",
            AssistantsApiToolChoiceOptionAutoEnum::Auto => "auto",
            AssistantsApiToolChoiceOptionAutoEnum::Required => "required",
        };
        write!(f, "{}", value)
    }
}
