use serde::{Deserialize, Serialize};

/// CreateImageEditRequestQualityTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreateImageEditRequestQualityTextVariantEnum {
    Low,
    Medium,
    High,
    Xhigh,
    Max,
    Auto,
    Standard,
}

impl std::fmt::Display for CreateImageEditRequestQualityTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            CreateImageEditRequestQualityTextVariantEnum::Low => "low",
            CreateImageEditRequestQualityTextVariantEnum::Medium => "medium",
            CreateImageEditRequestQualityTextVariantEnum::High => "high",
            CreateImageEditRequestQualityTextVariantEnum::Xhigh => "xhigh",
            CreateImageEditRequestQualityTextVariantEnum::Max => "max",
            CreateImageEditRequestQualityTextVariantEnum::Auto => "auto",
            CreateImageEditRequestQualityTextVariantEnum::Standard => "standard",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for CreateImageEditRequestQualityTextVariantEnum {
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
            _ => Err(crate::models::ParseEnumError::new(
                "CreateImageEditRequestQualityTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for CreateImageEditRequestQualityTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for CreateImageEditRequestQualityTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for CreateImageEditRequestQualityTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
