use crate::models;
use serde::{Deserialize, Serialize};

/// CreateImageEditRequestSize - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateImageEditRequestSize {
    Text(String),
    TextVariant(CreateImageEditRequestSizeTextVariantEnum),
}

impl Default for CreateImageEditRequestSize {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl CreateImageEditRequestSize {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for CreateImageEditRequestSize {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for CreateImageEditRequestSize {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for CreateImageEditRequestSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateImageEditRequestSize::Text(value) => write!(f, "{}", value),
            CreateImageEditRequestSize::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

/// CreateImageEditRequestSizeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreateImageEditRequestSizeTextVariantEnum {
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
    Auto,
}

impl Default for CreateImageEditRequestSizeTextVariantEnum {
    fn default() -> Self {
        Self::Variant256x256
    }
}

impl std::fmt::Display for CreateImageEditRequestSizeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            CreateImageEditRequestSizeTextVariantEnum::Variant256x256 => "256x256",
            CreateImageEditRequestSizeTextVariantEnum::Variant512x512 => "512x512",
            CreateImageEditRequestSizeTextVariantEnum::Variant1024x1024 => "1024x1024",
            CreateImageEditRequestSizeTextVariantEnum::Variant1536x1024 => "1536x1024",
            CreateImageEditRequestSizeTextVariantEnum::Variant1024x1536 => "1024x1536",
            CreateImageEditRequestSizeTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
