use serde::{Deserialize, Serialize};

/// ResponsePropertiesTruncationTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponsePropertiesTruncationTextVariantEnum {
    Auto,
    Disabled,
}

impl Default for ResponsePropertiesTruncationTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for ResponsePropertiesTruncationTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ResponsePropertiesTruncationTextVariantEnum::Auto => "auto",
            ResponsePropertiesTruncationTextVariantEnum::Disabled => "disabled",
        };
        write!(f, "{}", value)
    }
}
