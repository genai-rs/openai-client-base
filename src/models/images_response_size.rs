use crate::models;
use serde::{Deserialize, Serialize};

/// ImagesResponseSize - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImagesResponseSize {
    Text(String),
    TextVariant(ImagesResponseSizeTextVariantEnum),
}

impl Default for ImagesResponseSize {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl ImagesResponseSize {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for ImagesResponseSize {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for ImagesResponseSize {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for ImagesResponseSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImagesResponseSize::Text(value) => write!(f, "{}", value),
            ImagesResponseSize::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

/// ImagesResponseSizeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImagesResponseSizeTextVariantEnum {
    #[serde(rename = "1024x1024")]
    Variant1024x1024,
    #[serde(rename = "1024x1536")]
    Variant1024x1536,
    #[serde(rename = "1536x1024")]
    Variant1536x1024,
}

impl Default for ImagesResponseSizeTextVariantEnum {
    fn default() -> Self {
        Self::Variant1024x1024
    }
}

impl std::fmt::Display for ImagesResponseSizeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ImagesResponseSizeTextVariantEnum::Variant1024x1024 => "1024x1024",
            ImagesResponseSizeTextVariantEnum::Variant1024x1536 => "1024x1536",
            ImagesResponseSizeTextVariantEnum::Variant1536x1024 => "1536x1024",
        };
        write!(f, "{}", value)
    }
}
