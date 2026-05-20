use serde::{Deserialize, Serialize};

/// ImageGenToolSizeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageGenToolSizeTextVariantEnum {
    #[serde(rename = "1024x1024")]
    Variant1024x1024,
    #[serde(rename = "1024x1536")]
    Variant1024x1536,
    #[serde(rename = "1536x1024")]
    Variant1536x1024,
    Auto,
}

impl Default for ImageGenToolSizeTextVariantEnum {
    fn default() -> Self {
        Self::Variant1024x1024
    }
}

impl std::fmt::Display for ImageGenToolSizeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ImageGenToolSizeTextVariantEnum::Variant1024x1024 => "1024x1024",
            ImageGenToolSizeTextVariantEnum::Variant1024x1536 => "1024x1536",
            ImageGenToolSizeTextVariantEnum::Variant1536x1024 => "1536x1024",
            ImageGenToolSizeTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
