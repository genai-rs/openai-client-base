use serde::{Deserialize, Serialize};

/// BetaMisalignmentErrorType - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaMisalignmentErrorType {
    Text(String),
    TextVariant(BetaMisalignmentErrorTypeTextVariantEnum),
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

impl ::std::str::FromStr for BetaMisalignmentErrorTypeTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "potentially_unintended_data_transfer" => Ok(Self::PotentiallyUnintendedDataTransfer),
            "potentially_unintended_data_access" => Ok(Self::PotentiallyUnintendedDataAccess),
            "potentially_unintended_destructive_activity" => {
                Ok(Self::PotentiallyUnintendedDestructiveActivity)
            }
            "other" => Ok(Self::Other),
            _ => Err(crate::models::ParseEnumError::new(
                "BetaMisalignmentErrorTypeTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for BetaMisalignmentErrorTypeTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for BetaMisalignmentErrorTypeTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for BetaMisalignmentErrorTypeTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
