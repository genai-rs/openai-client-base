use crate::models;
use serde::{Deserialize, Serialize};

/// ImageEditCompletedEventSize - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageEditCompletedEventSize {
    Text(String),
    TextVariant(ImageEditCompletedEventSizeTextVariantEnum),
}

impl Default for ImageEditCompletedEventSize {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl ImageEditCompletedEventSize {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for ImageEditCompletedEventSize {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for ImageEditCompletedEventSize {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for ImageEditCompletedEventSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageEditCompletedEventSize::Text(value) => write!(f, "{}", value),
            ImageEditCompletedEventSize::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

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
