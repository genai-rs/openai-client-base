use serde::{Deserialize, Serialize};

/// EditImageBodyJsonParamSizeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EditImageBodyJsonParamSizeTextVariantEnum {
    Auto,
    #[serde(rename = "256x256")]
    Variant256x256,
    #[serde(rename = "512x512")]
    Variant512x512,
    #[serde(rename = "1024x1024")]
    Variant1024x1024,
    #[serde(rename = "1536x1024")]
    Variant1536x1024,
    #[serde(rename = "1024x1536")]
    Variant1024x1536,
}

impl Default for EditImageBodyJsonParamSizeTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for EditImageBodyJsonParamSizeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            EditImageBodyJsonParamSizeTextVariantEnum::Auto => "auto",
            EditImageBodyJsonParamSizeTextVariantEnum::Variant256x256 => "256x256",
            EditImageBodyJsonParamSizeTextVariantEnum::Variant512x512 => "512x512",
            EditImageBodyJsonParamSizeTextVariantEnum::Variant1024x1024 => "1024x1024",
            EditImageBodyJsonParamSizeTextVariantEnum::Variant1536x1024 => "1536x1024",
            EditImageBodyJsonParamSizeTextVariantEnum::Variant1024x1536 => "1024x1536",
        };
        write!(f, "{}", value)
    }
}
