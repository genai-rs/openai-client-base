use crate::models;
use serde::{Deserialize, Serialize};

/// AssistantsApiResponseFormatOption - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssistantsApiResponseFormatOption {
    TextVariant(AssistantsApiResponseFormatOptionTextVariantEnum),
    Responseformattext(models::ResponseFormatText),
    Responseformatjsonobject(models::ResponseFormatJsonObject),
    Responseformatjsonschema(models::ResponseFormatJsonSchema),
}

impl std::fmt::Display for AssistantsApiResponseFormatOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssistantsApiResponseFormatOption::TextVariant(value) => write!(f, "{}", value),
            AssistantsApiResponseFormatOption::Responseformattext(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            AssistantsApiResponseFormatOption::Responseformatjsonobject(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            AssistantsApiResponseFormatOption::Responseformatjsonschema(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}

/// AssistantsApiResponseFormatOptionTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssistantsApiResponseFormatOptionTextVariantEnum {
    Auto,
}

impl Default for AssistantsApiResponseFormatOptionTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for AssistantsApiResponseFormatOptionTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            AssistantsApiResponseFormatOptionTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
