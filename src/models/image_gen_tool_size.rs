use serde::{Deserialize, Serialize};

/// ImageGenToolSize - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageGenToolSize {
    Text(String),
    TextVariant(ImageGenToolSizeTextVariantEnum),
}

impl ImageGenToolSize {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for ImageGenToolSize {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for ImageGenToolSize {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for ImageGenToolSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageGenToolSize::Text(value) => write!(f, "{}", value),
            ImageGenToolSize::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

/// ImageGenToolSizeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageGenToolSizeTextVariantEnum {
    #[serde(rename = "1024x1024")]
    Variant1024x1024,
    #[serde(rename = "1024x1536")]
    Variant1024x1536,
    #[serde(rename = "1536x1024")]
    Variant1536x1024,
    Auto,
}

impl std::fmt::Display for ImageGenToolSizeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ImageGenToolSizeTextVariantEnum::Variant1024x1024 => "1024x1024",
            ImageGenToolSizeTextVariantEnum::Variant1024x1536 => "1024x1536",
            ImageGenToolSizeTextVariantEnum::Variant1536x1024 => "1536x1024",
            ImageGenToolSizeTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for ImageGenToolSizeTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "1024x1024" => Ok(Self::Variant1024x1024),
            "1024x1536" => Ok(Self::Variant1024x1536),
            "1536x1024" => Ok(Self::Variant1536x1024),
            "auto" => Ok(Self::Auto),
            _ => Err(crate::models::ParseEnumError::new(
                "ImageGenToolSizeTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for ImageGenToolSizeTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for ImageGenToolSizeTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for ImageGenToolSizeTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
