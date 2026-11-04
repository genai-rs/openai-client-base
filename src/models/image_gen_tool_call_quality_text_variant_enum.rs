use serde::{Deserialize, Serialize};

/// ImageGenToolCallQualityTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageGenToolCallQualityTextVariantEnum {
    Low,
    Medium,
    High,
    Xhigh,
    Max,
    Auto,
}

impl std::fmt::Display for ImageGenToolCallQualityTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ImageGenToolCallQualityTextVariantEnum::Low => "low",
            ImageGenToolCallQualityTextVariantEnum::Medium => "medium",
            ImageGenToolCallQualityTextVariantEnum::High => "high",
            ImageGenToolCallQualityTextVariantEnum::Xhigh => "xhigh",
            ImageGenToolCallQualityTextVariantEnum::Max => "max",
            ImageGenToolCallQualityTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for ImageGenToolCallQualityTextVariantEnum {
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
                "ImageGenToolCallQualityTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for ImageGenToolCallQualityTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for ImageGenToolCallQualityTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for ImageGenToolCallQualityTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
