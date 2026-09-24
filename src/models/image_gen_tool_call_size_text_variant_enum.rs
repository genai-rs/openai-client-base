use serde::{Deserialize, Serialize};

/// ImageGenToolCallSizeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageGenToolCallSizeTextVariantEnum {
    #[serde(rename = "1024x1024")]
    Variant1024x1024,
    #[serde(rename = "1024x1536")]
    Variant1024x1536,
    #[serde(rename = "1536x1024")]
    Variant1536x1024,
}

impl Default for ImageGenToolCallSizeTextVariantEnum {
    fn default() -> Self {
        Self::Variant1024x1024
    }
}

impl std::fmt::Display for ImageGenToolCallSizeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ImageGenToolCallSizeTextVariantEnum::Variant1024x1024 => "1024x1024",
            ImageGenToolCallSizeTextVariantEnum::Variant1024x1536 => "1024x1536",
            ImageGenToolCallSizeTextVariantEnum::Variant1536x1024 => "1536x1024",
        };
        write!(f, "{}", value)
    }
}
