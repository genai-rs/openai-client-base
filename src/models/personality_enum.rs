use crate::models;
use serde::{Deserialize, Serialize};

/// PersonalityEnum - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PersonalityEnum {
    Text(String),
    TextVariant(PersonalityEnumTextVariantEnum),
}

impl Default for PersonalityEnum {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl PersonalityEnum {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for PersonalityEnum {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for PersonalityEnum {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for PersonalityEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PersonalityEnum::Text(value) => write!(f, "{}", value),
            PersonalityEnum::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

/// PersonalityEnumTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PersonalityEnumTextVariantEnum {
    Friendly,
    Pragmatic,
}

impl Default for PersonalityEnumTextVariantEnum {
    fn default() -> Self {
        Self::Friendly
    }
}

impl std::fmt::Display for PersonalityEnumTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            PersonalityEnumTextVariantEnum::Friendly => "friendly",
            PersonalityEnumTextVariantEnum::Pragmatic => "pragmatic",
        };
        write!(f, "{}", value)
    }
}
