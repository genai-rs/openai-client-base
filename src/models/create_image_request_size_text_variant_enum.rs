use serde::{Deserialize, Serialize};

/// CreateImageRequestSizeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreateImageRequestSizeTextVariantEnum {
    Auto,
    #[serde(rename = "1024x1024")]
    Variant1024x1024,
    #[serde(rename = "1536x1024")]
    Variant1536x1024,
    #[serde(rename = "1024x1536")]
    Variant1024x1536,
    #[serde(rename = "256x256")]
    Variant256x256,
    #[serde(rename = "512x512")]
    Variant512x512,
    #[serde(rename = "1792x1024")]
    Variant1792x1024,
    #[serde(rename = "1024x1792")]
    Variant1024x1792,
}

impl std::fmt::Display for CreateImageRequestSizeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            CreateImageRequestSizeTextVariantEnum::Auto => "auto",
            CreateImageRequestSizeTextVariantEnum::Variant1024x1024 => "1024x1024",
            CreateImageRequestSizeTextVariantEnum::Variant1536x1024 => "1536x1024",
            CreateImageRequestSizeTextVariantEnum::Variant1024x1536 => "1024x1536",
            CreateImageRequestSizeTextVariantEnum::Variant256x256 => "256x256",
            CreateImageRequestSizeTextVariantEnum::Variant512x512 => "512x512",
            CreateImageRequestSizeTextVariantEnum::Variant1792x1024 => "1792x1024",
            CreateImageRequestSizeTextVariantEnum::Variant1024x1792 => "1024x1792",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for CreateImageRequestSizeTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "auto" => Ok(Self::Auto),
            "1024x1024" => Ok(Self::Variant1024x1024),
            "1536x1024" => Ok(Self::Variant1536x1024),
            "1024x1536" => Ok(Self::Variant1024x1536),
            "256x256" => Ok(Self::Variant256x256),
            "512x512" => Ok(Self::Variant512x512),
            "1792x1024" => Ok(Self::Variant1792x1024),
            "1024x1792" => Ok(Self::Variant1024x1792),
            _ => Err(crate::models::ParseEnumError::new(
                "CreateImageRequestSizeTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for CreateImageRequestSizeTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for CreateImageRequestSizeTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for CreateImageRequestSizeTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
