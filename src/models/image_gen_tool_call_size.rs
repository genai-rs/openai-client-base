use crate::models;
use serde::{Deserialize, Serialize};

/// ImageGenToolCallSize - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageGenToolCallSize {
    Text(String),
    TextVariant(ImageGenToolCallSizeTextVariantEnum),
    Null,
}

impl Default for ImageGenToolCallSize {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl ImageGenToolCallSize {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for ImageGenToolCallSize {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for ImageGenToolCallSize {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for ImageGenToolCallSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageGenToolCallSize::Text(value) => write!(f, "{}", value),
            ImageGenToolCallSize::TextVariant(value) => write!(f, "{}", value),
            ImageGenToolCallSize::Null => write!(f, "null"),
        }
    }
}

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
