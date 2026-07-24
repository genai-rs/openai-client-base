use crate::models;
use serde::{Deserialize, Serialize};

/// BetaServiceTier - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaServiceTier {
    TextVariant(BetaServiceTierTextVariantEnum),
    Null,
}

impl std::fmt::Display for BetaServiceTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaServiceTier::TextVariant(value) => write!(f, "{}", value),
            BetaServiceTier::Null => write!(f, "null"),
        }
    }
}

/// BetaServiceTierTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaServiceTierTextVariantEnum {
    Auto,
    Default,
    Flex,
    Scale,
    Priority,
}

impl Default for BetaServiceTierTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for BetaServiceTierTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaServiceTierTextVariantEnum::Auto => "auto",
            BetaServiceTierTextVariantEnum::Default => "default",
            BetaServiceTierTextVariantEnum::Flex => "flex",
            BetaServiceTierTextVariantEnum::Scale => "scale",
            BetaServiceTierTextVariantEnum::Priority => "priority",
        };
        write!(f, "{}", value)
    }
}
