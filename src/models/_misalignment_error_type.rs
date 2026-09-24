use crate::models;
use serde::{Deserialize, Serialize};

/// MisalignmentErrorType - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MisalignmentErrorType {
    Text(String),
    TextVariant(MisalignmentErrorTypeTextVariantEnum),
}

impl Default for MisalignmentErrorType {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl MisalignmentErrorType {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for MisalignmentErrorType {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for MisalignmentErrorType {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for MisalignmentErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MisalignmentErrorType::Text(value) => write!(f, "{}", value),
            MisalignmentErrorType::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

/// MisalignmentErrorTypeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MisalignmentErrorTypeTextVariantEnum {
    #[serde(rename = "potentially_unintended_data_transfer")]
    PotentiallyUnintendedDataTransfer,
    #[serde(rename = "potentially_unintended_data_access")]
    PotentiallyUnintendedDataAccess,
    #[serde(rename = "potentially_unintended_destructive_activity")]
    PotentiallyUnintendedDestructiveActivity,
    Other,
}

impl Default for MisalignmentErrorTypeTextVariantEnum {
    fn default() -> Self {
        Self::PotentiallyUnintendedDataTransfer
    }
}

impl std::fmt::Display for MisalignmentErrorTypeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            MisalignmentErrorTypeTextVariantEnum::PotentiallyUnintendedDataTransfer => {
                "potentially_unintended_data_transfer"
            }
            MisalignmentErrorTypeTextVariantEnum::PotentiallyUnintendedDataAccess => {
                "potentially_unintended_data_access"
            }
            MisalignmentErrorTypeTextVariantEnum::PotentiallyUnintendedDestructiveActivity => {
                "potentially_unintended_destructive_activity"
            }
            MisalignmentErrorTypeTextVariantEnum::Other => "other",
        };
        write!(f, "{}", value)
    }
}
