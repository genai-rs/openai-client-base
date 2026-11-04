use serde::{Deserialize, Serialize};

/// UsageWebSearchCallsResultApiSourceTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UsageWebSearchCallsResultApiSourceTextVariantEnum {
    #[serde(rename = "agents_api")]
    AgentsApi,
    Unlabeled,
}

impl std::fmt::Display for UsageWebSearchCallsResultApiSourceTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            UsageWebSearchCallsResultApiSourceTextVariantEnum::AgentsApi => "agents_api",
            UsageWebSearchCallsResultApiSourceTextVariantEnum::Unlabeled => "unlabeled",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for UsageWebSearchCallsResultApiSourceTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "agents_api" => Ok(Self::AgentsApi),
            "unlabeled" => Ok(Self::Unlabeled),
            _ => Err(crate::models::ParseEnumError::new(
                "UsageWebSearchCallsResultApiSourceTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for UsageWebSearchCallsResultApiSourceTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for UsageWebSearchCallsResultApiSourceTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for UsageWebSearchCallsResultApiSourceTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
