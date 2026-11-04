use serde::{Deserialize, Serialize};

/// CreateImageEditRequestBackgroundTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreateImageEditRequestBackgroundTextVariantEnum {
    Transparent,
    Opaque,
    Auto,
}

impl std::fmt::Display for CreateImageEditRequestBackgroundTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            CreateImageEditRequestBackgroundTextVariantEnum::Transparent => "transparent",
            CreateImageEditRequestBackgroundTextVariantEnum::Opaque => "opaque",
            CreateImageEditRequestBackgroundTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for CreateImageEditRequestBackgroundTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "transparent" => Ok(Self::Transparent),
            "opaque" => Ok(Self::Opaque),
            "auto" => Ok(Self::Auto),
            _ => Err(crate::models::ParseEnumError::new(
                "CreateImageEditRequestBackgroundTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for CreateImageEditRequestBackgroundTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for CreateImageEditRequestBackgroundTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for CreateImageEditRequestBackgroundTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
