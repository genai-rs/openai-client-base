use serde::{Deserialize, Serialize};

/// ServiceTierResponsesTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceTierResponsesTextVariantEnum {
    Auto,
    Default,
    Flex,
    Scale,
    Priority,
    Fast,
    Ultrafast,
}

impl std::fmt::Display for ServiceTierResponsesTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ServiceTierResponsesTextVariantEnum::Auto => "auto",
            ServiceTierResponsesTextVariantEnum::Default => "default",
            ServiceTierResponsesTextVariantEnum::Flex => "flex",
            ServiceTierResponsesTextVariantEnum::Scale => "scale",
            ServiceTierResponsesTextVariantEnum::Priority => "priority",
            ServiceTierResponsesTextVariantEnum::Fast => "fast",
            ServiceTierResponsesTextVariantEnum::Ultrafast => "ultrafast",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for ServiceTierResponsesTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "auto" => Ok(Self::Auto),
            "default" => Ok(Self::Default),
            "flex" => Ok(Self::Flex),
            "scale" => Ok(Self::Scale),
            "priority" => Ok(Self::Priority),
            "fast" => Ok(Self::Fast),
            "ultrafast" => Ok(Self::Ultrafast),
            _ => Err(crate::models::ParseEnumError::new(
                "ServiceTierResponsesTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for ServiceTierResponsesTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for ServiceTierResponsesTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for ServiceTierResponsesTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
