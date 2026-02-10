use serde::{Deserialize, Serialize};

/// AssistantsApiToolChoiceOptionTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssistantsApiToolChoiceOptionTextVariantEnum {
    None,
    Auto,
    Required,
}

impl Default for AssistantsApiToolChoiceOptionTextVariantEnum {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for AssistantsApiToolChoiceOptionTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            AssistantsApiToolChoiceOptionTextVariantEnum::None => "none",
            AssistantsApiToolChoiceOptionTextVariantEnum::Auto => "auto",
            AssistantsApiToolChoiceOptionTextVariantEnum::Required => "required",
        };
        write!(f, "{}", value)
    }
}
