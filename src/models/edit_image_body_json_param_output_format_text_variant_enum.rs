use serde::{Deserialize, Serialize};

/// EditImageBodyJsonParamOutputFormatTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EditImageBodyJsonParamOutputFormatTextVariantEnum {
    Png,
    Jpeg,
    Webp,
}

impl std::fmt::Display for EditImageBodyJsonParamOutputFormatTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            EditImageBodyJsonParamOutputFormatTextVariantEnum::Png => "png",
            EditImageBodyJsonParamOutputFormatTextVariantEnum::Jpeg => "jpeg",
            EditImageBodyJsonParamOutputFormatTextVariantEnum::Webp => "webp",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for EditImageBodyJsonParamOutputFormatTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "png" => Ok(Self::Png),
            "jpeg" => Ok(Self::Jpeg),
            "webp" => Ok(Self::Webp),
            _ => Err(crate::models::ParseEnumError::new(
                "EditImageBodyJsonParamOutputFormatTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for EditImageBodyJsonParamOutputFormatTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for EditImageBodyJsonParamOutputFormatTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for EditImageBodyJsonParamOutputFormatTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
