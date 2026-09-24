use serde::{Deserialize, Serialize};

/// BetaImageGenToolCallQualityTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaImageGenToolCallQualityTextVariantEnum {
    Low,
    Medium,
    High,
    Xhigh,
    Max,
    Auto,
}

impl Default for BetaImageGenToolCallQualityTextVariantEnum {
    fn default() -> Self {
        Self::Low
    }
}

impl std::fmt::Display for BetaImageGenToolCallQualityTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaImageGenToolCallQualityTextVariantEnum::Low => "low",
            BetaImageGenToolCallQualityTextVariantEnum::Medium => "medium",
            BetaImageGenToolCallQualityTextVariantEnum::High => "high",
            BetaImageGenToolCallQualityTextVariantEnum::Xhigh => "xhigh",
            BetaImageGenToolCallQualityTextVariantEnum::Max => "max",
            BetaImageGenToolCallQualityTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
