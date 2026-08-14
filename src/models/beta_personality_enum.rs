use crate::models;
use serde::{Deserialize, Serialize};

/// BetaPersonalityEnum - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaPersonalityEnum {
    Text(String),
    TextVariant(BetaPersonalityEnumTextVariantEnum),
}

impl Default for BetaPersonalityEnum {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl BetaPersonalityEnum {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for BetaPersonalityEnum {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for BetaPersonalityEnum {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for BetaPersonalityEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaPersonalityEnum::Text(value) => write!(f, "{}", value),
            BetaPersonalityEnum::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

/// BetaPersonalityEnumTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaPersonalityEnumTextVariantEnum {
    Friendly,
    Pragmatic,
}

impl Default for BetaPersonalityEnumTextVariantEnum {
    fn default() -> Self {
        Self::Friendly
    }
}

impl std::fmt::Display for BetaPersonalityEnumTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaPersonalityEnumTextVariantEnum::Friendly => "friendly",
            BetaPersonalityEnumTextVariantEnum::Pragmatic => "pragmatic",
        };
        write!(f, "{}", value)
    }
}
