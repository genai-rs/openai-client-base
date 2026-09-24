use serde::{Deserialize, Serialize};

/// ImageGenToolCallQualityTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageGenToolCallQualityTextVariantEnum {
    Low,
    Medium,
    High,
    Xhigh,
    Max,
    Auto,
}

impl Default for ImageGenToolCallQualityTextVariantEnum {
    fn default() -> Self {
        Self::Low
    }
}

impl std::fmt::Display for ImageGenToolCallQualityTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ImageGenToolCallQualityTextVariantEnum::Low => "low",
            ImageGenToolCallQualityTextVariantEnum::Medium => "medium",
            ImageGenToolCallQualityTextVariantEnum::High => "high",
            ImageGenToolCallQualityTextVariantEnum::Xhigh => "xhigh",
            ImageGenToolCallQualityTextVariantEnum::Max => "max",
            ImageGenToolCallQualityTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
