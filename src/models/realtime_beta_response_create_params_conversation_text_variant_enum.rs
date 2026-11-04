use serde::{Deserialize, Serialize};

/// RealtimeBetaResponseCreateParamsConversationTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeBetaResponseCreateParamsConversationTextVariantEnum {
    Auto,
    None,
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

impl ::std::str::FromStr for RealtimeBetaResponseCreateParamsConversationTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "auto" => Ok(Self::Auto),
            "none" => Ok(Self::None),
            _ => Err(crate::models::ParseEnumError::new(
                "RealtimeBetaResponseCreateParamsConversationTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for RealtimeBetaResponseCreateParamsConversationTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for RealtimeBetaResponseCreateParamsConversationTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeBetaResponseCreateParamsConversationTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
