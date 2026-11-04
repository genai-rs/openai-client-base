use serde::{Deserialize, Serialize};

/// RealtimeSessionCreateRequestGaTracingAutoEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeSessionCreateRequestGaTracingAutoEnum {
    Auto,
}

impl std::fmt::Display for RealtimeSessionCreateRequestGaTracingAutoEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RealtimeSessionCreateRequestGaTracingAutoEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for RealtimeSessionCreateRequestGaTracingAutoEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err(crate::models::ParseEnumError::new(
                "RealtimeSessionCreateRequestGaTracingAutoEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for RealtimeSessionCreateRequestGaTracingAutoEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for RealtimeSessionCreateRequestGaTracingAutoEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeSessionCreateRequestGaTracingAutoEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
