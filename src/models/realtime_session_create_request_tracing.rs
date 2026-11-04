use serde::{Deserialize, Serialize};

/// RealtimeSessionCreateRequestTracing - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeSessionCreateRequestTracing {
    TextVariant(RealtimeSessionCreateRequestTracingTextVariantEnum),
}

impl std::fmt::Display for RealtimeSessionCreateRequestTracing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RealtimeSessionCreateRequestTracing::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

/// RealtimeSessionCreateRequestTracingTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeSessionCreateRequestTracingTextVariantEnum {
    Auto,
}

impl std::fmt::Display for RealtimeSessionCreateRequestTracingTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RealtimeSessionCreateRequestTracingTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for RealtimeSessionCreateRequestTracingTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err(crate::models::ParseEnumError::new(
                "RealtimeSessionCreateRequestTracingTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for RealtimeSessionCreateRequestTracingTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for RealtimeSessionCreateRequestTracingTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeSessionCreateRequestTracingTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
