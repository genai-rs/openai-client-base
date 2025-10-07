use crate::models;
use serde::{Deserialize, Serialize};

/// AssistantsApiToolChoiceOption - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssistantsApiToolChoiceOption {
    Auto(AssistantsApiToolChoiceOptionAutoEnum),
    Assistantsnamedtoolchoice(models::AssistantsNamedToolChoice),
}

impl std::fmt::Display for AssistantsApiToolChoiceOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssistantsApiToolChoiceOption::Auto(value) => write!(f, "{}", value),
            AssistantsApiToolChoiceOption::Assistantsnamedtoolchoice(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}

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
