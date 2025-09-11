use serde::{Deserialize, Serialize};

/// AssistantsApiResponseFormatOptionText - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssistantsApiResponseFormatOptionText {
    Text,
}

impl Default for AssistantsApiResponseFormatOptionText {
    fn default() -> Self {
        Self::Text
    }
}
