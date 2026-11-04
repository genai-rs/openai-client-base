use serde::{Deserialize, Serialize};

/// CreateImageEditRequestOutputFormatTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreateImageEditRequestOutputFormatTextVariantEnum {
    Png,
    Jpeg,
    Webp,
}

impl std::fmt::Display for CreateImageEditRequestOutputFormatTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            CreateImageEditRequestOutputFormatTextVariantEnum::Png => "png",
            CreateImageEditRequestOutputFormatTextVariantEnum::Jpeg => "jpeg",
            CreateImageEditRequestOutputFormatTextVariantEnum::Webp => "webp",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for CreateImageEditRequestOutputFormatTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "png" => Ok(Self::Png),
            "jpeg" => Ok(Self::Jpeg),
            "webp" => Ok(Self::Webp),
            _ => Err(crate::models::ParseEnumError::new(
                "CreateImageEditRequestOutputFormatTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for CreateImageEditRequestOutputFormatTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for CreateImageEditRequestOutputFormatTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for CreateImageEditRequestOutputFormatTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
