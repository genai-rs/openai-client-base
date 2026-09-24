use crate::models;
use serde::{Deserialize, Serialize};

/// BetaMisalignmentErrorType - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaMisalignmentErrorType {
    Text(String),
    TextVariant(BetaMisalignmentErrorTypeTextVariantEnum),
}

impl Default for BetaMisalignmentErrorType {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl BetaMisalignmentErrorType {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for BetaMisalignmentErrorType {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for BetaMisalignmentErrorType {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for BetaMisalignmentErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaMisalignmentErrorType::Text(value) => write!(f, "{}", value),
            BetaMisalignmentErrorType::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

/// BetaMisalignmentErrorTypeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaMisalignmentErrorTypeTextVariantEnum {
    #[serde(rename = "potentially_unintended_data_transfer")]
    PotentiallyUnintendedDataTransfer,
    #[serde(rename = "potentially_unintended_data_access")]
    PotentiallyUnintendedDataAccess,
    #[serde(rename = "potentially_unintended_destructive_activity")]
    PotentiallyUnintendedDestructiveActivity,
    Other,
}

impl Default for BetaMisalignmentErrorTypeTextVariantEnum {
    fn default() -> Self {
        Self::PotentiallyUnintendedDataTransfer
    }
}

impl std::fmt::Display for BetaMisalignmentErrorTypeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaMisalignmentErrorTypeTextVariantEnum::PotentiallyUnintendedDataTransfer => {
                "potentially_unintended_data_transfer"
            }
            BetaMisalignmentErrorTypeTextVariantEnum::PotentiallyUnintendedDataAccess => {
                "potentially_unintended_data_access"
            }
            BetaMisalignmentErrorTypeTextVariantEnum::PotentiallyUnintendedDestructiveActivity => {
                "potentially_unintended_destructive_activity"
            }
            BetaMisalignmentErrorTypeTextVariantEnum::Other => "other",
        };
        write!(f, "{}", value)
    }
}
