use serde::{Deserialize, Serialize};

/// CreateImageRequestOutputFormatTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreateImageRequestOutputFormatTextVariantEnum {
    Png,
    Jpeg,
    Webp,
}

impl std::fmt::Display for CreateImageRequestOutputFormatTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            CreateImageRequestOutputFormatTextVariantEnum::Png => "png",
            CreateImageRequestOutputFormatTextVariantEnum::Jpeg => "jpeg",
            CreateImageRequestOutputFormatTextVariantEnum::Webp => "webp",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for CreateImageRequestOutputFormatTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "png" => Ok(Self::Png),
            "jpeg" => Ok(Self::Jpeg),
            "webp" => Ok(Self::Webp),
            _ => Err(crate::models::ParseEnumError::new(
                "CreateImageRequestOutputFormatTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for CreateImageRequestOutputFormatTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for CreateImageRequestOutputFormatTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for CreateImageRequestOutputFormatTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
