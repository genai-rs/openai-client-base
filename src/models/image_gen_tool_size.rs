use crate::models;
use serde::{Deserialize, Serialize};

/// ImageGenToolSize - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageGenToolSize {
    Text(String),
    TextVariant(ImageGenToolSizeTextVariantEnum),
}

impl Default for ImageGenToolSize {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl ImageGenToolSize {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for ImageGenToolSize {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for ImageGenToolSize {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for ImageGenToolSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageGenToolSize::Text(value) => write!(f, "{}", value),
            ImageGenToolSize::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

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
