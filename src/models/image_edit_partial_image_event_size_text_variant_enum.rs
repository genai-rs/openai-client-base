use serde::{Deserialize, Serialize};

/// ImageEditPartialImageEventSizeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageEditPartialImageEventSizeTextVariantEnum {
    #[serde(rename = "1024x1024")]
    Variant1024x1024,
    #[serde(rename = "1024x1536")]
    Variant1024x1536,
    #[serde(rename = "1536x1024")]
    Variant1536x1024,
    Auto,
}

impl Default for ImageEditPartialImageEventSizeTextVariantEnum {
    fn default() -> Self {
        Self::Variant1024x1024
    }
}

impl std::fmt::Display for ImageEditPartialImageEventSizeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ImageEditPartialImageEventSizeTextVariantEnum::Variant1024x1024 => "1024x1024",
            ImageEditPartialImageEventSizeTextVariantEnum::Variant1024x1536 => "1024x1536",
            ImageEditPartialImageEventSizeTextVariantEnum::Variant1536x1024 => "1536x1024",
            ImageEditPartialImageEventSizeTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
