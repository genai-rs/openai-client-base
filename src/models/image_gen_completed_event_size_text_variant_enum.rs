use serde::{Deserialize, Serialize};

/// ImageGenCompletedEventSizeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageGenCompletedEventSizeTextVariantEnum {
    #[serde(rename = "1024x1024")]
    Variant1024x1024,
    #[serde(rename = "1024x1536")]
    Variant1024x1536,
    #[serde(rename = "1536x1024")]
    Variant1536x1024,
    Auto,
}

impl Default for ImageGenCompletedEventSizeTextVariantEnum {
    fn default() -> Self {
        Self::Variant1024x1024
    }
}

impl std::fmt::Display for ImageGenCompletedEventSizeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ImageGenCompletedEventSizeTextVariantEnum::Variant1024x1024 => "1024x1024",
            ImageGenCompletedEventSizeTextVariantEnum::Variant1024x1536 => "1024x1536",
            ImageGenCompletedEventSizeTextVariantEnum::Variant1536x1024 => "1536x1024",
            ImageGenCompletedEventSizeTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
