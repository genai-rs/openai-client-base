use serde::{Deserialize, Serialize};

/// EditImageBodyJsonParamQualityTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EditImageBodyJsonParamQualityTextVariantEnum {
    Low,
    Medium,
    High,
    Xhigh,
    Max,
    Auto,
}

impl Default for EditImageBodyJsonParamQualityTextVariantEnum {
    fn default() -> Self {
        Self::Low
    }
}

impl std::fmt::Display for EditImageBodyJsonParamQualityTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            EditImageBodyJsonParamQualityTextVariantEnum::Low => "low",
            EditImageBodyJsonParamQualityTextVariantEnum::Medium => "medium",
            EditImageBodyJsonParamQualityTextVariantEnum::High => "high",
            EditImageBodyJsonParamQualityTextVariantEnum::Xhigh => "xhigh",
            EditImageBodyJsonParamQualityTextVariantEnum::Max => "max",
            EditImageBodyJsonParamQualityTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
