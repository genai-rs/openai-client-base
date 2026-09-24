use serde::{Deserialize, Serialize};

/// ImageEditCompletedEventSizeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageEditCompletedEventSizeTextVariantEnum {
    #[serde(rename = "1024x1024")]
    Variant1024x1024,
    #[serde(rename = "1024x1536")]
    Variant1024x1536,
    #[serde(rename = "1536x1024")]
    Variant1536x1024,
    Auto,
}

impl Default for ImageEditCompletedEventSizeTextVariantEnum {
    fn default() -> Self {
        Self::Variant1024x1024
    }
}

impl std::fmt::Display for ImageEditCompletedEventSizeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ImageEditCompletedEventSizeTextVariantEnum::Variant1024x1024 => "1024x1024",
            ImageEditCompletedEventSizeTextVariantEnum::Variant1024x1536 => "1024x1536",
            ImageEditCompletedEventSizeTextVariantEnum::Variant1536x1024 => "1536x1024",
            ImageEditCompletedEventSizeTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
