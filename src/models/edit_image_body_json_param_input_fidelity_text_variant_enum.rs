use serde::{Deserialize, Serialize};

/// EditImageBodyJsonParamInputFidelityTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EditImageBodyJsonParamInputFidelityTextVariantEnum {
    High,
    Low,
}

impl Default for EditImageBodyJsonParamInputFidelityTextVariantEnum {
    fn default() -> Self {
        Self::High
    }
}

impl std::fmt::Display for EditImageBodyJsonParamInputFidelityTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            EditImageBodyJsonParamInputFidelityTextVariantEnum::High => "high",
            EditImageBodyJsonParamInputFidelityTextVariantEnum::Low => "low",
        };
        write!(f, "{}", value)
    }
}
