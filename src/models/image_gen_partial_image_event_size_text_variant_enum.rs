use serde::{Deserialize, Serialize};

/// ImageGenPartialImageEventSizeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageGenPartialImageEventSizeTextVariantEnum {
    #[serde(rename = "1024x1024")]
    Variant1024x1024,
    #[serde(rename = "1024x1536")]
    Variant1024x1536,
    #[serde(rename = "1536x1024")]
    Variant1536x1024,
    Auto,
}

impl std::fmt::Display for ImageGenPartialImageEventSizeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ImageGenPartialImageEventSizeTextVariantEnum::Variant1024x1024 => "1024x1024",
            ImageGenPartialImageEventSizeTextVariantEnum::Variant1024x1536 => "1024x1536",
            ImageGenPartialImageEventSizeTextVariantEnum::Variant1536x1024 => "1536x1024",
            ImageGenPartialImageEventSizeTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for ImageGenPartialImageEventSizeTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "1024x1024" => Ok(Self::Variant1024x1024),
            "1024x1536" => Ok(Self::Variant1024x1536),
            "1536x1024" => Ok(Self::Variant1536x1024),
            "auto" => Ok(Self::Auto),
            _ => Err(crate::models::ParseEnumError::new(
                "ImageGenPartialImageEventSizeTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for ImageGenPartialImageEventSizeTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for ImageGenPartialImageEventSizeTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for ImageGenPartialImageEventSizeTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
