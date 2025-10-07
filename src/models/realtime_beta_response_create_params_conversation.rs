use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeBetaResponseCreateParamsConversation - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeBetaResponseCreateParamsConversation {
    Text(String),
    TextVariant(RealtimeBetaResponseCreateParamsConversationTextVariantEnum),
}

impl Default for RealtimeBetaResponseCreateParamsConversation {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl RealtimeBetaResponseCreateParamsConversation {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for RealtimeBetaResponseCreateParamsConversation {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for RealtimeBetaResponseCreateParamsConversation {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for RealtimeBetaResponseCreateParamsConversation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RealtimeBetaResponseCreateParamsConversation::Text(value) => write!(f, "{}", value),
            RealtimeBetaResponseCreateParamsConversation::TextVariant(value) => {
                write!(f, "{}", value)
            }
        }
    }
}

/// RealtimeBetaResponseCreateParamsConversationTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeBetaResponseCreateParamsConversationTextVariantEnum {
    Auto,
    None,
}

impl Default for RealtimeBetaResponseCreateParamsConversationTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for RealtimeBetaResponseCreateParamsConversationTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RealtimeBetaResponseCreateParamsConversationTextVariantEnum::Auto => "auto",
            RealtimeBetaResponseCreateParamsConversationTextVariantEnum::None => "none",
        };
        write!(f, "{}", value)
    }
}
