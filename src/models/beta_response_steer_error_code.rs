use crate::models;
use serde::{Deserialize, Serialize};

/// BetaResponseSteerErrorCode - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaResponseSteerErrorCode {
    TextVariant(BetaResponseSteerErrorCodeTextVariantEnum),
    Text(String),
}

impl std::fmt::Display for BetaResponseSteerErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaResponseSteerErrorCode::TextVariant(value) => write!(f, "{}", value),
            BetaResponseSteerErrorCode::Text(value) => write!(f, "{}", value),
        }
    }
}

/// BetaResponseSteerErrorCodeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaResponseSteerErrorCodeTextVariantEnum {
    #[serde(rename = "response_not_found")]
    ResponseNotFound,
    #[serde(rename = "invalid_input")]
    InvalidInput,
    #[serde(rename = "steering_not_supported")]
    SteeringNotSupported,
    #[serde(rename = "too_many_pending_steers")]
    TooManyPendingSteers,
    #[serde(rename = "response_already_completed")]
    ResponseAlreadyCompleted,
    #[serde(rename = "response_not_active")]
    ResponseNotActive,
    #[serde(rename = "successor_creation_failed")]
    SuccessorCreationFailed,
}

impl Default for BetaResponseSteerErrorCodeTextVariantEnum {
    fn default() -> Self {
        Self::ResponseNotFound
    }
}

impl std::fmt::Display for BetaResponseSteerErrorCodeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaResponseSteerErrorCodeTextVariantEnum::ResponseNotFound => "response_not_found",
            BetaResponseSteerErrorCodeTextVariantEnum::InvalidInput => "invalid_input",
            BetaResponseSteerErrorCodeTextVariantEnum::SteeringNotSupported => {
                "steering_not_supported"
            }
            BetaResponseSteerErrorCodeTextVariantEnum::TooManyPendingSteers => {
                "too_many_pending_steers"
            }
            BetaResponseSteerErrorCodeTextVariantEnum::ResponseAlreadyCompleted => {
                "response_already_completed"
            }
            BetaResponseSteerErrorCodeTextVariantEnum::ResponseNotActive => "response_not_active",
            BetaResponseSteerErrorCodeTextVariantEnum::SuccessorCreationFailed => {
                "successor_creation_failed"
            }
        };
        write!(f, "{}", value)
    }
}
