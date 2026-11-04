use serde::{Deserialize, Serialize};

/// CreateImageEditRequestSizeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreateImageEditRequestSizeTextVariantEnum {
    #[serde(rename = "1024x1024")]
    Variant1024x1024,
    #[serde(rename = "1536x1024")]
    Variant1536x1024,
    #[serde(rename = "1024x1536")]
    Variant1024x1536,
    Auto,
    #[serde(rename = "256x256")]
    Variant256x256,
    #[serde(rename = "512x512")]
    Variant512x512,
}

impl std::fmt::Display for CreateImageEditRequestSizeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            CreateImageEditRequestSizeTextVariantEnum::Variant1024x1024 => "1024x1024",
            CreateImageEditRequestSizeTextVariantEnum::Variant1536x1024 => "1536x1024",
            CreateImageEditRequestSizeTextVariantEnum::Variant1024x1536 => "1024x1536",
            CreateImageEditRequestSizeTextVariantEnum::Auto => "auto",
            CreateImageEditRequestSizeTextVariantEnum::Variant256x256 => "256x256",
            CreateImageEditRequestSizeTextVariantEnum::Variant512x512 => "512x512",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for CreateImageEditRequestSizeTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "1024x1024" => Ok(Self::Variant1024x1024),
            "1536x1024" => Ok(Self::Variant1536x1024),
            "1024x1536" => Ok(Self::Variant1024x1536),
            "auto" => Ok(Self::Auto),
            "256x256" => Ok(Self::Variant256x256),
            "512x512" => Ok(Self::Variant512x512),
            _ => Err(crate::models::ParseEnumError::new(
                "CreateImageEditRequestSizeTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for CreateImageEditRequestSizeTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for CreateImageEditRequestSizeTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for CreateImageEditRequestSizeTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
