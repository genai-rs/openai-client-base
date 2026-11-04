use serde::{Deserialize, Serialize};

/// BetaServiceTierResponses - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaServiceTierResponses {
    TextVariant(BetaServiceTierResponsesTextVariantEnum),
    Null,
}

impl std::fmt::Display for BetaServiceTierResponses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaServiceTierResponses::TextVariant(value) => write!(f, "{}", value),
            BetaServiceTierResponses::Null => write!(f, "null"),
        }
    }
}

/// BetaServiceTierResponsesTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaServiceTierResponsesTextVariantEnum {
    Auto,
    Default,
    Flex,
    Scale,
    Priority,
    Fast,
    Ultrafast,
}

impl std::fmt::Display for BetaServiceTierResponsesTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaServiceTierResponsesTextVariantEnum::Auto => "auto",
            BetaServiceTierResponsesTextVariantEnum::Default => "default",
            BetaServiceTierResponsesTextVariantEnum::Flex => "flex",
            BetaServiceTierResponsesTextVariantEnum::Scale => "scale",
            BetaServiceTierResponsesTextVariantEnum::Priority => "priority",
            BetaServiceTierResponsesTextVariantEnum::Fast => "fast",
            BetaServiceTierResponsesTextVariantEnum::Ultrafast => "ultrafast",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for BetaServiceTierResponsesTextVariantEnum {
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
                "BetaServiceTierResponsesTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for BetaServiceTierResponsesTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for BetaServiceTierResponsesTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for BetaServiceTierResponsesTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
