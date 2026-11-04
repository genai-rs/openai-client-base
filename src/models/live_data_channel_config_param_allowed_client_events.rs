use serde::{Deserialize, Serialize};

/// LiveDataChannelConfigParamAllowedClientEvents - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LiveDataChannelConfigParamAllowedClientEvents {
    TextVariant(LiveDataChannelConfigParamAllowedClientEventsTextVariantEnum),
    ArrayOfStrings(Vec<String>),
}

impl std::fmt::Display for LiveDataChannelConfigParamAllowedClientEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiveDataChannelConfigParamAllowedClientEvents::TextVariant(value) => {
                write!(f, "{}", value)
            }
            LiveDataChannelConfigParamAllowedClientEvents::ArrayOfStrings(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}

/// LiveDataChannelConfigParamAllowedClientEventsTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LiveDataChannelConfigParamAllowedClientEventsTextVariantEnum {
    All,
}

impl std::fmt::Display for LiveDataChannelConfigParamAllowedClientEventsTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            LiveDataChannelConfigParamAllowedClientEventsTextVariantEnum::All => "all",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for LiveDataChannelConfigParamAllowedClientEventsTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "all" => Ok(Self::All),
            _ => Err(crate::models::ParseEnumError::new(
                "LiveDataChannelConfigParamAllowedClientEventsTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str>
    for LiveDataChannelConfigParamAllowedClientEventsTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for LiveDataChannelConfigParamAllowedClientEventsTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for LiveDataChannelConfigParamAllowedClientEventsTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
