use crate::models;
use serde::{Deserialize, Serialize};

/// ResponseSteerErrorCode - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseSteerErrorCode {
    TextVariant(ResponseSteerErrorCodeTextVariantEnum),
    Text(String),
}

impl std::fmt::Display for ResponseSteerErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseSteerErrorCode::TextVariant(value) => write!(f, "{}", value),
            ResponseSteerErrorCode::Text(value) => write!(f, "{}", value),
        }
    }
}

/// ResponseSteerErrorCodeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponseSteerErrorCodeTextVariantEnum {
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

impl Default for ResponseSteerErrorCodeTextVariantEnum {
    fn default() -> Self {
        Self::ResponseNotFound
    }
}

impl std::fmt::Display for ResponseSteerErrorCodeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ResponseSteerErrorCodeTextVariantEnum::ResponseNotFound => "response_not_found",
            ResponseSteerErrorCodeTextVariantEnum::InvalidInput => "invalid_input",
            ResponseSteerErrorCodeTextVariantEnum::SteeringNotSupported => "steering_not_supported",
            ResponseSteerErrorCodeTextVariantEnum::TooManyPendingSteers => {
                "too_many_pending_steers"
            }
            ResponseSteerErrorCodeTextVariantEnum::ResponseAlreadyCompleted => {
                "response_already_completed"
            }
            ResponseSteerErrorCodeTextVariantEnum::ResponseNotActive => "response_not_active",
            ResponseSteerErrorCodeTextVariantEnum::SuccessorCreationFailed => {
                "successor_creation_failed"
            }
        };
        write!(f, "{}", value)
    }
}
