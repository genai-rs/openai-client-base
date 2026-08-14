use serde::{Deserialize, Serialize};

/// BetaReasoningModeEnumTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaReasoningModeEnumTextVariantEnum {
    Standard,
    Pro,
}

impl Default for BetaReasoningModeEnumTextVariantEnum {
    fn default() -> Self {
        Self::Standard
    }
}

impl std::fmt::Display for BetaReasoningModeEnumTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaReasoningModeEnumTextVariantEnum::Standard => "standard",
            BetaReasoningModeEnumTextVariantEnum::Pro => "pro",
        };
        write!(f, "{}", value)
    }
}
