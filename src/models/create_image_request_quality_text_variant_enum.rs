use serde::{Deserialize, Serialize};

/// CreateImageRequestQualityTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreateImageRequestQualityTextVariantEnum {
    Low,
    Medium,
    High,
    Xhigh,
    Max,
    Auto,
    Standard,
    Hd,
}

impl std::fmt::Display for CreateImageRequestQualityTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            CreateImageRequestQualityTextVariantEnum::Low => "low",
            CreateImageRequestQualityTextVariantEnum::Medium => "medium",
            CreateImageRequestQualityTextVariantEnum::High => "high",
            CreateImageRequestQualityTextVariantEnum::Xhigh => "xhigh",
            CreateImageRequestQualityTextVariantEnum::Max => "max",
            CreateImageRequestQualityTextVariantEnum::Auto => "auto",
            CreateImageRequestQualityTextVariantEnum::Standard => "standard",
            CreateImageRequestQualityTextVariantEnum::Hd => "hd",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for CreateImageRequestQualityTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "xhigh" => Ok(Self::Xhigh),
            "max" => Ok(Self::Max),
            "auto" => Ok(Self::Auto),
            "standard" => Ok(Self::Standard),
            "hd" => Ok(Self::Hd),
            _ => Err(crate::models::ParseEnumError::new(
                "CreateImageRequestQualityTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for CreateImageRequestQualityTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for CreateImageRequestQualityTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for CreateImageRequestQualityTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
