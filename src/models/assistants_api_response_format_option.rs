use crate::models;
use serde::{Deserialize, Serialize};

/// AssistantsApiResponseFormatOption - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssistantsApiResponseFormatOption {
    Text(TextEnum),
    Responseformattext(models::ResponseFormatText),
    Responseformatjsonobject(models::ResponseFormatJsonObject),
    Responseformatjsonschema(models::ResponseFormatJsonSchema),
}

impl Default for AssistantsApiResponseFormatOption {
    fn default() -> Self {
        Self::Text(Default::default())
    }
}

