use serde::{Deserialize, Serialize};

/// ReasoningModeEnumTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningModeEnumTextVariantEnum {
    Standard,
    Pro,
}

impl Default for ReasoningModeEnumTextVariantEnum {
    fn default() -> Self {
        Self::Standard
    }
}

impl std::fmt::Display for ReasoningModeEnumTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ReasoningModeEnumTextVariantEnum::Standard => "standard",
            ReasoningModeEnumTextVariantEnum::Pro => "pro",
        };
        write!(f, "{}", value)
    }
}
