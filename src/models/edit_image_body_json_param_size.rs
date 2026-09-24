use crate::models;
use serde::{Deserialize, Serialize};

/// EditImageBodyJsonParamSize - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditImageBodyJsonParamSize {
    Text(String),
    TextVariant(EditImageBodyJsonParamSizeTextVariantEnum),
    Null,
}

impl Default for EditImageBodyJsonParamSize {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl EditImageBodyJsonParamSize {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for EditImageBodyJsonParamSize {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for EditImageBodyJsonParamSize {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for EditImageBodyJsonParamSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditImageBodyJsonParamSize::Text(value) => write!(f, "{}", value),
            EditImageBodyJsonParamSize::TextVariant(value) => write!(f, "{}", value),
            EditImageBodyJsonParamSize::Null => write!(f, "null"),
        }
    }
}

/// EditImageBodyJsonParamSizeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EditImageBodyJsonParamSizeTextVariantEnum {
    Auto,
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
            EditImageBodyJsonParamSizeTextVariantEnum::Variant1024x1024 => "1024x1024",
            EditImageBodyJsonParamSizeTextVariantEnum::Variant1536x1024 => "1536x1024",
            EditImageBodyJsonParamSizeTextVariantEnum::Variant1024x1536 => "1024x1536",
        };
        write!(f, "{}", value)
    }
}
