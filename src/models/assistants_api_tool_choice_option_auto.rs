use serde::{Deserialize, Serialize};

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
