use serde::{Deserialize, Serialize};

/// RealtimeResponseMaxOutputTokensTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeResponseMaxOutputTokensTextVariantEnum {
    Inf,
}

impl std::fmt::Display for RealtimeResponseMaxOutputTokensTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RealtimeResponseMaxOutputTokensTextVariantEnum::Inf => "inf",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for RealtimeResponseMaxOutputTokensTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "inf" => Ok(Self::Inf),
            _ => Err(crate::models::ParseEnumError::new(
                "RealtimeResponseMaxOutputTokensTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for RealtimeResponseMaxOutputTokensTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for RealtimeResponseMaxOutputTokensTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeResponseMaxOutputTokensTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
