use serde::{Deserialize, Serialize};

/// EditImageBodyJsonParamQualityTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EditImageBodyJsonParamQualityTextVariantEnum {
    Standard,
    Low,
    Medium,
    High,
    Auto,
}

impl Default for EditImageBodyJsonParamQualityTextVariantEnum {
    fn default() -> Self {
        Self::Standard
    }
}

impl std::fmt::Display for EditImageBodyJsonParamQualityTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            EditImageBodyJsonParamQualityTextVariantEnum::Standard => "standard",
            EditImageBodyJsonParamQualityTextVariantEnum::Low => "low",
            EditImageBodyJsonParamQualityTextVariantEnum::Medium => "medium",
            EditImageBodyJsonParamQualityTextVariantEnum::High => "high",
            EditImageBodyJsonParamQualityTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
