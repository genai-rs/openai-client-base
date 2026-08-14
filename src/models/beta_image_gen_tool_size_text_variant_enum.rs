use serde::{Deserialize, Serialize};

/// BetaImageGenToolSizeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaImageGenToolSizeTextVariantEnum {
    #[serde(rename = "1024x1024")]
    Variant1024x1024,
    #[serde(rename = "1024x1536")]
    Variant1024x1536,
    #[serde(rename = "1536x1024")]
    Variant1536x1024,
    Auto,
}

impl Default for BetaImageGenToolSizeTextVariantEnum {
    fn default() -> Self {
        Self::Variant1024x1024
    }
}

impl std::fmt::Display for BetaImageGenToolSizeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaImageGenToolSizeTextVariantEnum::Variant1024x1024 => "1024x1024",
            BetaImageGenToolSizeTextVariantEnum::Variant1024x1536 => "1024x1536",
            BetaImageGenToolSizeTextVariantEnum::Variant1536x1024 => "1536x1024",
            BetaImageGenToolSizeTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
