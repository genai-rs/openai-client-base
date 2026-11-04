use serde::{Deserialize, Serialize};

/// BetaResponseSteerPendingReason - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaResponseSteerPendingReason {
    TextVariant(BetaResponseSteerPendingReasonTextVariantEnum),
    Text(String),
}

impl std::fmt::Display for BetaResponseSteerPendingReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaResponseSteerPendingReason::TextVariant(value) => write!(f, "{}", value),
            BetaResponseSteerPendingReason::Text(value) => write!(f, "{}", value),
        }
    }
}

/// BetaResponseSteerPendingReasonTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaResponseSteerPendingReasonTextVariantEnum {
    #[serde(rename = "waiting_for_required_input")]
    WaitingForRequiredInput,
}

impl std::fmt::Display for BetaResponseSteerPendingReasonTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaResponseSteerPendingReasonTextVariantEnum::WaitingForRequiredInput => {
                "waiting_for_required_input"
            }
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for BetaResponseSteerPendingReasonTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "waiting_for_required_input" => Ok(Self::WaitingForRequiredInput),
            _ => Err(crate::models::ParseEnumError::new(
                "BetaResponseSteerPendingReasonTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for BetaResponseSteerPendingReasonTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for BetaResponseSteerPendingReasonTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for BetaResponseSteerPendingReasonTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
