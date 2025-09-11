use serde::{Deserialize, Serialize};

/// AssistantsApiResponseFormatOptionAuto - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssistantsApiResponseFormatOptionAuto {
    Auto,
}

impl Default for AssistantsApiResponseFormatOptionAuto {
    fn default() -> Self {
        Self::Auto
    }
}
