use serde::{Deserialize, Serialize};

/// BetaImageGenToolCallSizeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaImageGenToolCallSizeTextVariantEnum {
    #[serde(rename = "1024x1024")]
    Variant1024x1024,
    #[serde(rename = "1024x1536")]
    Variant1024x1536,
    #[serde(rename = "1536x1024")]
    Variant1536x1024,
}

impl Default for BetaImageGenToolCallSizeTextVariantEnum {
    fn default() -> Self {
        Self::Variant1024x1024
    }
}

impl std::fmt::Display for BetaImageGenToolCallSizeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaImageGenToolCallSizeTextVariantEnum::Variant1024x1024 => "1024x1024",
            BetaImageGenToolCallSizeTextVariantEnum::Variant1024x1536 => "1024x1536",
            BetaImageGenToolCallSizeTextVariantEnum::Variant1536x1024 => "1536x1024",
        };
        write!(f, "{}", value)
    }
}
