use serde::{Deserialize, Serialize};

/// CreateImageRequestModerationTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreateImageRequestModerationTextVariantEnum {
    Low,
    Auto,
}

impl Default for CreateImageRequestModerationTextVariantEnum {
    fn default() -> Self {
        Self::Low
    }
}

impl std::fmt::Display for CreateImageRequestModerationTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            CreateImageRequestModerationTextVariantEnum::Low => "low",
            CreateImageRequestModerationTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
