use crate::models;
use serde::{Deserialize, Serialize};

/// AssistantsApiToolChoiceOption - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssistantsApiToolChoiceOption {
    Auto(AssistantsApiToolChoiceOptionAuto),
    Named(models::AssistantsNamedToolChoice),
}

impl Default for AssistantsApiToolChoiceOption {
    fn default() -> Self {
        Self::Auto(Default::default())
    }
}

/// AssistantsApiToolChoiceOptionAuto - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssistantsApiToolChoiceOptionAuto {
    None,
    Auto,
    Required,
}

impl Default for AssistantsApiToolChoiceOptionAuto {
    fn default() -> Self {
        Self::None
    }
}
