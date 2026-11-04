use serde::{Deserialize, Serialize};

/// BetaImageGenToolCallQualityTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaImageGenToolCallQualityTextVariantEnum {
    Low,
    Medium,
    High,
    Xhigh,
    Max,
    Auto,
}

impl std::fmt::Display for BetaImageGenToolCallQualityTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaImageGenToolCallQualityTextVariantEnum::Low => "low",
            BetaImageGenToolCallQualityTextVariantEnum::Medium => "medium",
            BetaImageGenToolCallQualityTextVariantEnum::High => "high",
            BetaImageGenToolCallQualityTextVariantEnum::Xhigh => "xhigh",
            BetaImageGenToolCallQualityTextVariantEnum::Max => "max",
            BetaImageGenToolCallQualityTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for BetaImageGenToolCallQualityTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "xhigh" => Ok(Self::Xhigh),
            "max" => Ok(Self::Max),
            "auto" => Ok(Self::Auto),
            _ => Err(crate::models::ParseEnumError::new(
                "BetaImageGenToolCallQualityTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for BetaImageGenToolCallQualityTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for BetaImageGenToolCallQualityTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for BetaImageGenToolCallQualityTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
