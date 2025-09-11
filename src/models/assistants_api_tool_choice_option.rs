use crate::models;
use serde::{Deserialize, Serialize};

/// AssistantsApiToolChoiceOption - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssistantsApiToolChoiceOption {
    Auto(AutoEnum),
    Assistantsnamedtoolchoice(models::AssistantsNamedToolChoice),
}

impl Default for AssistantsApiToolChoiceOption {
    fn default() -> Self {
        Self::Auto(Default::default())
    }
}

