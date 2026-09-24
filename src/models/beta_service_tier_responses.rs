use crate::models;
use serde::{Deserialize, Serialize};

/// BetaServiceTierResponses - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaServiceTierResponses {
    TextVariant(BetaServiceTierResponsesTextVariantEnum),
    Null,
}

impl std::fmt::Display for BetaServiceTierResponses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaServiceTierResponses::TextVariant(value) => write!(f, "{}", value),
            BetaServiceTierResponses::Null => write!(f, "null"),
        }
    }
}

/// BetaServiceTierResponsesTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaServiceTierResponsesTextVariantEnum {
    Auto,
    Default,
    Flex,
    Scale,
    Priority,
    Fast,
    Ultrafast,
}

impl Default for BetaServiceTierResponsesTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for BetaServiceTierResponsesTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaServiceTierResponsesTextVariantEnum::Auto => "auto",
            BetaServiceTierResponsesTextVariantEnum::Default => "default",
            BetaServiceTierResponsesTextVariantEnum::Flex => "flex",
            BetaServiceTierResponsesTextVariantEnum::Scale => "scale",
            BetaServiceTierResponsesTextVariantEnum::Priority => "priority",
            BetaServiceTierResponsesTextVariantEnum::Fast => "fast",
            BetaServiceTierResponsesTextVariantEnum::Ultrafast => "ultrafast",
        };
        write!(f, "{}", value)
    }
}
