use serde::{Deserialize, Serialize};

/// EditImageBodyJsonParamSizeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EditImageBodyJsonParamSizeTextVariantEnum {
    Auto,
    #[serde(rename = "1024x1024")]
    Variant1024x1024,
    #[serde(rename = "1536x1024")]
    Variant1536x1024,
    #[serde(rename = "1024x1536")]
    Variant1024x1536,
}

impl std::fmt::Display for EditImageBodyJsonParamSizeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            EditImageBodyJsonParamSizeTextVariantEnum::Auto => "auto",
            EditImageBodyJsonParamSizeTextVariantEnum::Variant1024x1024 => "1024x1024",
            EditImageBodyJsonParamSizeTextVariantEnum::Variant1536x1024 => "1536x1024",
            EditImageBodyJsonParamSizeTextVariantEnum::Variant1024x1536 => "1024x1536",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for EditImageBodyJsonParamSizeTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "auto" => Ok(Self::Auto),
            "1024x1024" => Ok(Self::Variant1024x1024),
            "1536x1024" => Ok(Self::Variant1536x1024),
            "1024x1536" => Ok(Self::Variant1024x1536),
            _ => Err(crate::models::ParseEnumError::new(
                "EditImageBodyJsonParamSizeTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for EditImageBodyJsonParamSizeTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for EditImageBodyJsonParamSizeTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for EditImageBodyJsonParamSizeTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
