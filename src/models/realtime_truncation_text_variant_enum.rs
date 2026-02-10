use serde::{Deserialize, Serialize};

/// RealtimeTruncationTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeTruncationTextVariantEnum {
    Auto,
    Disabled,
}

impl Default for RealtimeTruncationTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for RealtimeTruncationTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RealtimeTruncationTextVariantEnum::Auto => "auto",
            RealtimeTruncationTextVariantEnum::Disabled => "disabled",
        };
        write!(f, "{}", value)
    }
}
