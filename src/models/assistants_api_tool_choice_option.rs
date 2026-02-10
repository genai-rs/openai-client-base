use crate::models;
use serde::{Deserialize, Serialize};

/// AssistantsApiToolChoiceOption - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssistantsApiToolChoiceOption {
    TextVariant(AssistantsApiToolChoiceOptionTextVariantEnum),
    Assistantsnamedtoolchoice(models::AssistantsNamedToolChoice),
}

impl std::fmt::Display for AssistantsApiToolChoiceOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssistantsApiToolChoiceOption::TextVariant(value) => write!(f, "{}", value),
            AssistantsApiToolChoiceOption::Assistantsnamedtoolchoice(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}

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
