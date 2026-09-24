use serde::{Deserialize, Serialize};

/// ImageGenPartialImageEventSizeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageGenPartialImageEventSizeTextVariantEnum {
    #[serde(rename = "1024x1024")]
    Variant1024x1024,
    #[serde(rename = "1024x1536")]
    Variant1024x1536,
    #[serde(rename = "1536x1024")]
    Variant1536x1024,
    Auto,
}

impl Default for ImageGenPartialImageEventSizeTextVariantEnum {
    fn default() -> Self {
        Self::Variant1024x1024
    }
}

impl std::fmt::Display for ImageGenPartialImageEventSizeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ImageGenPartialImageEventSizeTextVariantEnum::Variant1024x1024 => "1024x1024",
            ImageGenPartialImageEventSizeTextVariantEnum::Variant1024x1536 => "1024x1536",
            ImageGenPartialImageEventSizeTextVariantEnum::Variant1536x1024 => "1536x1024",
            ImageGenPartialImageEventSizeTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
