use serde::{Deserialize, Serialize};

/// EditImageBodyJsonParamInputFidelityTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EditImageBodyJsonParamInputFidelityTextVariantEnum {
    High,
    Low,
}

impl std::fmt::Display for EditImageBodyJsonParamInputFidelityTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            EditImageBodyJsonParamInputFidelityTextVariantEnum::High => "high",
            EditImageBodyJsonParamInputFidelityTextVariantEnum::Low => "low",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for EditImageBodyJsonParamInputFidelityTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "high" => Ok(Self::High),
            "low" => Ok(Self::Low),
            _ => Err(crate::models::ParseEnumError::new(
                "EditImageBodyJsonParamInputFidelityTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for EditImageBodyJsonParamInputFidelityTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for EditImageBodyJsonParamInputFidelityTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for EditImageBodyJsonParamInputFidelityTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
