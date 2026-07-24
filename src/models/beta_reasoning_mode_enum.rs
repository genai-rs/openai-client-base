use crate::models;
use serde::{Deserialize, Serialize};

/// BetaReasoningModeEnum - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaReasoningModeEnum {
    Text(String),
    TextVariant(BetaReasoningModeEnumTextVariantEnum),
}

impl Default for BetaReasoningModeEnum {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl BetaReasoningModeEnum {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for BetaReasoningModeEnum {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for BetaReasoningModeEnum {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for BetaReasoningModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaReasoningModeEnum::Text(value) => write!(f, "{}", value),
            BetaReasoningModeEnum::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

/// BetaReasoningModeEnumTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaReasoningModeEnumTextVariantEnum {
    Standard,
    Pro,
}

impl Default for BetaReasoningModeEnumTextVariantEnum {
    fn default() -> Self {
        Self::Standard
    }
}

impl std::fmt::Display for BetaReasoningModeEnumTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaReasoningModeEnumTextVariantEnum::Standard => "standard",
            BetaReasoningModeEnumTextVariantEnum::Pro => "pro",
        };
        write!(f, "{}", value)
    }
}
