use serde::{Deserialize, Serialize};

/// RealtimeBetaResponseMaxOutputTokens - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeBetaResponseMaxOutputTokens {
    TextVariant(RealtimeBetaResponseMaxOutputTokensTextVariantEnum),
}

impl std::fmt::Display for RealtimeBetaResponseMaxOutputTokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RealtimeBetaResponseMaxOutputTokens::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

/// RealtimeBetaResponseMaxOutputTokensTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeBetaResponseMaxOutputTokensTextVariantEnum {
    Inf,
}

impl std::fmt::Display for RealtimeBetaResponseMaxOutputTokensTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RealtimeBetaResponseMaxOutputTokensTextVariantEnum::Inf => "inf",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for RealtimeBetaResponseMaxOutputTokensTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "inf" => Ok(Self::Inf),
            _ => Err(crate::models::ParseEnumError::new(
                "RealtimeBetaResponseMaxOutputTokensTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for RealtimeBetaResponseMaxOutputTokensTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for RealtimeBetaResponseMaxOutputTokensTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeBetaResponseMaxOutputTokensTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
