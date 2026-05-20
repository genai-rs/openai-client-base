use serde::{Deserialize, Serialize};

/// CreateImageRequestSizeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreateImageRequestSizeTextVariantEnum {
    Auto,
    #[serde(rename = "1024x1024")]
    Variant1024x1024,
    #[serde(rename = "1536x1024")]
    Variant1536x1024,
    #[serde(rename = "1024x1536")]
    Variant1024x1536,
    #[serde(rename = "256x256")]
    Variant256x256,
    #[serde(rename = "512x512")]
    Variant512x512,
    #[serde(rename = "1792x1024")]
    Variant1792x1024,
    #[serde(rename = "1024x1792")]
    Variant1024x1792,
}

impl Default for CreateImageRequestSizeTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for CreateImageRequestSizeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            CreateImageRequestSizeTextVariantEnum::Auto => "auto",
            CreateImageRequestSizeTextVariantEnum::Variant1024x1024 => "1024x1024",
            CreateImageRequestSizeTextVariantEnum::Variant1536x1024 => "1536x1024",
            CreateImageRequestSizeTextVariantEnum::Variant1024x1536 => "1024x1536",
            CreateImageRequestSizeTextVariantEnum::Variant256x256 => "256x256",
            CreateImageRequestSizeTextVariantEnum::Variant512x512 => "512x512",
            CreateImageRequestSizeTextVariantEnum::Variant1792x1024 => "1792x1024",
            CreateImageRequestSizeTextVariantEnum::Variant1024x1792 => "1024x1792",
        };
        write!(f, "{}", value)
    }
}
