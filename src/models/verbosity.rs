use crate::models;
use serde::{Deserialize, Serialize};

/// Verbosity - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Verbosity {
    TextVariant(VerbosityTextVariantEnum),
    Null,
}

impl std::fmt::Display for Verbosity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Verbosity::TextVariant(value) => write!(f, "{}", value),
            Verbosity::Null => write!(f, "null"),
        }
    }
}

/// VerbosityTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VerbosityTextVariantEnum {
    Low,
    Medium,
    High,
}

impl Default for VerbosityTextVariantEnum {
    fn default() -> Self {
        Self::Low
    }
}

impl std::fmt::Display for VerbosityTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            VerbosityTextVariantEnum::Low => "low",
            VerbosityTextVariantEnum::Medium => "medium",
            VerbosityTextVariantEnum::High => "high",
        };
        write!(f, "{}", value)
    }
}
