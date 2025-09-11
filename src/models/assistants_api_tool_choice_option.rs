use crate::models;
use serde::{Deserialize, Serialize};

/// AssistantsApiToolChoiceOption - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssistantsApiToolChoiceOption {
    Auto(AutoEnum),
    Assistantsnamedtoolchoice(models::AssistantsNamedToolChoice),
}

/// AutoEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AutoEnum {
    None,
    Auto,
    Required,
}

impl Default for AutoEnum {
    fn default() -> Self {
        Self::None
    }
}
