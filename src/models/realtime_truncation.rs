use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeTruncation - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeTruncation {
    TextVariant(RealtimeTruncationTextVariantEnum),
    RetentionRatioTruncation(models::ChatCompletionNamedToolChoice),
}

impl std::fmt::Display for RealtimeTruncation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RealtimeTruncation::TextVariant(value) => write!(f, "{}", value),
            RealtimeTruncation::RetentionRatioTruncation(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}

/// RealtimeTruncationTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeTruncationTextVariantEnum {
    Auto,
    Disabled,
}

impl Default for RealtimeTruncationTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for RealtimeTruncationTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RealtimeTruncationTextVariantEnum::Auto => "auto",
            RealtimeTruncationTextVariantEnum::Disabled => "disabled",
        };
        write!(f, "{}", value)
    }
}
