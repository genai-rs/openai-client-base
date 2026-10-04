use serde::{Deserialize, Serialize};

/// CreateImageRequestQualityTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreateImageRequestQualityTextVariantEnum {
    Low,
    Medium,
    High,
    Xhigh,
    Max,
    Auto,
    Standard,
    Hd,
}

impl Default for CreateImageRequestQualityTextVariantEnum {
    fn default() -> Self {
        Self::Low
    }
}

impl std::fmt::Display for CreateImageRequestQualityTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            CreateImageRequestQualityTextVariantEnum::Low => "low",
            CreateImageRequestQualityTextVariantEnum::Medium => "medium",
            CreateImageRequestQualityTextVariantEnum::High => "high",
            CreateImageRequestQualityTextVariantEnum::Xhigh => "xhigh",
            CreateImageRequestQualityTextVariantEnum::Max => "max",
            CreateImageRequestQualityTextVariantEnum::Auto => "auto",
            CreateImageRequestQualityTextVariantEnum::Standard => "standard",
            CreateImageRequestQualityTextVariantEnum::Hd => "hd",
        };
        write!(f, "{}", value)
    }
}
