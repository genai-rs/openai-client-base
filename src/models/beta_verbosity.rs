use crate::models;
use serde::{Deserialize, Serialize};

/// BetaVerbosity - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaVerbosity {
    TextVariant(BetaVerbosityTextVariantEnum),
    Null,
}

impl std::fmt::Display for BetaVerbosity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaVerbosity::TextVariant(value) => write!(f, "{}", value),
            BetaVerbosity::Null => write!(f, "null"),
        }
    }
}

/// BetaVerbosityTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaVerbosityTextVariantEnum {
    Low,
    Medium,
    High,
}

impl Default for BetaVerbosityTextVariantEnum {
    fn default() -> Self {
        Self::Low
    }
}

impl std::fmt::Display for BetaVerbosityTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaVerbosityTextVariantEnum::Low => "low",
            BetaVerbosityTextVariantEnum::Medium => "medium",
            BetaVerbosityTextVariantEnum::High => "high",
        };
        write!(f, "{}", value)
    }
}
