use serde::{Deserialize, Serialize};

/// ServiceTierTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceTierTextVariantEnum {
    Auto,
    Default,
    Flex,
    Scale,
    Priority,
    Fast,
}

impl std::fmt::Display for ServiceTierTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ServiceTierTextVariantEnum::Auto => "auto",
            ServiceTierTextVariantEnum::Default => "default",
            ServiceTierTextVariantEnum::Flex => "flex",
            ServiceTierTextVariantEnum::Scale => "scale",
            ServiceTierTextVariantEnum::Priority => "priority",
            ServiceTierTextVariantEnum::Fast => "fast",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for ServiceTierTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "auto" => Ok(Self::Auto),
            "default" => Ok(Self::Default),
            "flex" => Ok(Self::Flex),
            "scale" => Ok(Self::Scale),
            "priority" => Ok(Self::Priority),
            "fast" => Ok(Self::Fast),
            _ => Err(crate::models::ParseEnumError::new(
                "ServiceTierTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for ServiceTierTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for ServiceTierTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for ServiceTierTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
