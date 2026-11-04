use serde::{Deserialize, Serialize};

/// RealtimeBetaResponseCreateParamsMaxOutputTokens - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeBetaResponseCreateParamsMaxOutputTokens {
    TextVariant(RealtimeBetaResponseCreateParamsMaxOutputTokensTextVariantEnum),
}

impl std::fmt::Display for RealtimeBetaResponseCreateParamsMaxOutputTokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RealtimeBetaResponseCreateParamsMaxOutputTokens::TextVariant(value) => {
                write!(f, "{}", value)
            }
        }
    }
}

/// RealtimeBetaResponseCreateParamsMaxOutputTokensTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeBetaResponseCreateParamsMaxOutputTokensTextVariantEnum {
    Inf,
}

impl std::fmt::Display for RealtimeBetaResponseCreateParamsMaxOutputTokensTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RealtimeBetaResponseCreateParamsMaxOutputTokensTextVariantEnum::Inf => "inf",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for RealtimeBetaResponseCreateParamsMaxOutputTokensTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "inf" => Ok(Self::Inf),
            _ => Err(crate::models::ParseEnumError::new(
                "RealtimeBetaResponseCreateParamsMaxOutputTokensTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str>
    for RealtimeBetaResponseCreateParamsMaxOutputTokensTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for RealtimeBetaResponseCreateParamsMaxOutputTokensTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeBetaResponseCreateParamsMaxOutputTokensTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
