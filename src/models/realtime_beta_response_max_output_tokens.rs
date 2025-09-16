use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeBetaResponseMaxOutputTokens - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeBetaResponseMaxOutputTokens {
    TextVariant(RealtimeBetaResponseMaxOutputTokensTextVariantEnum),
}

impl std::fmt::Display for RealtimeBetaResponseMaxOutputTokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RealtimeBetaResponseMaxOutputTokens::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

/// RealtimeBetaResponseMaxOutputTokensTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeBetaResponseMaxOutputTokensTextVariantEnum {
    Inf,
}

impl Default for RealtimeBetaResponseMaxOutputTokensTextVariantEnum {
    fn default() -> Self {
        Self::Inf
    }
}

impl std::fmt::Display for RealtimeBetaResponseMaxOutputTokensTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RealtimeBetaResponseMaxOutputTokensTextVariantEnum::Inf => "inf",
        };
        write!(f, "{}", value)
    }
}
