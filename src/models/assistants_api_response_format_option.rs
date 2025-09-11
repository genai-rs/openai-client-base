use crate::models;
use serde::{Deserialize, Serialize};

/// AssistantsApiResponseFormatOption - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssistantsApiResponseFormatOption {
    Auto(AssistantsApiResponseFormatOptionAuto),
    Text(AssistantsApiResponseFormatOptionText),
    JsonObject(models::ResponseFormatJsonObject),
    JsonSchema(models::ResponseFormatJsonSchema),
}

impl Default for AssistantsApiResponseFormatOption {
    fn default() -> Self {
        Self::Auto(Default::default())
    }
}

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
