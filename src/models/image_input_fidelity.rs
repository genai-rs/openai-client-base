use crate::models;
use serde::{Deserialize, Serialize};

/// ImageInputFidelity - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageInputFidelity {
    TextVariant(ImageInputFidelityTextVariantEnum),
    Null,
}

impl std::fmt::Display for ImageInputFidelity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageInputFidelity::TextVariant(value) => write!(f, "{}", value),
            ImageInputFidelity::Null => write!(f, "null"),
        }
    }
}

/// ImageInputFidelityTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageInputFidelityTextVariantEnum {
    High,
    Low,
}

impl Default for ImageInputFidelityTextVariantEnum {
    fn default() -> Self {
        Self::High
    }
}

impl std::fmt::Display for ImageInputFidelityTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ImageInputFidelityTextVariantEnum::High => "high",
            ImageInputFidelityTextVariantEnum::Low => "low",
        };
        write!(f, "{}", value)
    }
}
