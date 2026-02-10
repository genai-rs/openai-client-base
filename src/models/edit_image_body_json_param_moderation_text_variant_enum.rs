use serde::{Deserialize, Serialize};

/// EditImageBodyJsonParamModerationTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EditImageBodyJsonParamModerationTextVariantEnum {
    Low,
    Auto,
}

impl Default for EditImageBodyJsonParamModerationTextVariantEnum {
    fn default() -> Self {
        Self::Low
    }
}

impl std::fmt::Display for EditImageBodyJsonParamModerationTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            EditImageBodyJsonParamModerationTextVariantEnum::Low => "low",
            EditImageBodyJsonParamModerationTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
