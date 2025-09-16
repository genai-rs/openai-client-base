use serde::{Deserialize, Serialize};

/// VerbosityTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VerbosityTextVariantEnum {
    Low,
    Medium,
    High,
}

impl Default for VerbosityTextVariantEnum {
    fn default() -> Self {
        Self::Low
    }
}

impl std::fmt::Display for VerbosityTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            VerbosityTextVariantEnum::Low => "low",
            VerbosityTextVariantEnum::Medium => "medium",
            VerbosityTextVariantEnum::High => "high",
        };
        write!(f, "{}", value)
    }
}
