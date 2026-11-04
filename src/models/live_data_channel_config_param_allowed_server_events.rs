use crate::models;
use serde::{Deserialize, Serialize};

/// LiveDataChannelConfigParamAllowedServerEvents - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LiveDataChannelConfigParamAllowedServerEvents {
    TextVariant(LiveDataChannelConfigParamAllowedServerEventsTextVariantEnum),
    Arrayofliveallowedservereventparams(Vec<models::LiveAllowedServerEventParam>),
}

impl std::fmt::Display for LiveDataChannelConfigParamAllowedServerEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiveDataChannelConfigParamAllowedServerEvents::TextVariant(value) => {
                write!(f, "{}", value)
            }
            LiveDataChannelConfigParamAllowedServerEvents::Arrayofliveallowedservereventparams(
                value,
            ) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}

/// LiveDataChannelConfigParamAllowedServerEventsTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LiveDataChannelConfigParamAllowedServerEventsTextVariantEnum {
    All,
}

impl std::fmt::Display for LiveDataChannelConfigParamAllowedServerEventsTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            LiveDataChannelConfigParamAllowedServerEventsTextVariantEnum::All => "all",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for LiveDataChannelConfigParamAllowedServerEventsTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "all" => Ok(Self::All),
            _ => Err(crate::models::ParseEnumError::new(
                "LiveDataChannelConfigParamAllowedServerEventsTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str>
    for LiveDataChannelConfigParamAllowedServerEventsTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for LiveDataChannelConfigParamAllowedServerEventsTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for LiveDataChannelConfigParamAllowedServerEventsTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
