use serde::{Deserialize, Serialize};

/// CreateImageEditRequestQualityTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreateImageEditRequestQualityTextVariantEnum {
    Low,
    Medium,
    High,
    Xhigh,
    Max,
    Auto,
    Standard,
}

impl Default for CreateImageEditRequestQualityTextVariantEnum {
    fn default() -> Self {
        Self::Low
    }
}

impl std::fmt::Display for CreateImageEditRequestQualityTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            CreateImageEditRequestQualityTextVariantEnum::Low => "low",
            CreateImageEditRequestQualityTextVariantEnum::Medium => "medium",
            CreateImageEditRequestQualityTextVariantEnum::High => "high",
            CreateImageEditRequestQualityTextVariantEnum::Xhigh => "xhigh",
            CreateImageEditRequestQualityTextVariantEnum::Max => "max",
            CreateImageEditRequestQualityTextVariantEnum::Auto => "auto",
            CreateImageEditRequestQualityTextVariantEnum::Standard => "standard",
        };
        write!(f, "{}", value)
    }
}
