use serde::{Deserialize, Serialize};

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

impl ::std::str::FromStr for ResponseSteerErrorCodeTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "response_not_found" => Ok(Self::ResponseNotFound),
            "invalid_input" => Ok(Self::InvalidInput),
            "steering_not_supported" => Ok(Self::SteeringNotSupported),
            "too_many_pending_steers" => Ok(Self::TooManyPendingSteers),
            "response_already_completed" => Ok(Self::ResponseAlreadyCompleted),
            "response_not_active" => Ok(Self::ResponseNotActive),
            "successor_creation_failed" => Ok(Self::SuccessorCreationFailed),
            _ => Err(crate::models::ParseEnumError::new(
                "ResponseSteerErrorCodeTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for ResponseSteerErrorCodeTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for ResponseSteerErrorCodeTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for ResponseSteerErrorCodeTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
