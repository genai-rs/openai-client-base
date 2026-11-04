use serde::{Deserialize, Serialize};

/// ImageEditCompletedEventSize - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageEditCompletedEventSize {
    Text(String),
    TextVariant(ImageEditCompletedEventSizeTextVariantEnum),
}

impl ImageEditCompletedEventSize {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for ImageEditCompletedEventSize {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for ImageEditCompletedEventSize {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for ImageEditCompletedEventSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageEditCompletedEventSize::Text(value) => write!(f, "{}", value),
            ImageEditCompletedEventSize::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

/// ImageEditCompletedEventSizeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageEditCompletedEventSizeTextVariantEnum {
    #[serde(rename = "1024x1024")]
    Variant1024x1024,
    #[serde(rename = "1024x1536")]
    Variant1024x1536,
    #[serde(rename = "1536x1024")]
    Variant1536x1024,
    Auto,
}

impl std::fmt::Display for ImageEditCompletedEventSizeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ImageEditCompletedEventSizeTextVariantEnum::Variant1024x1024 => "1024x1024",
            ImageEditCompletedEventSizeTextVariantEnum::Variant1024x1536 => "1024x1536",
            ImageEditCompletedEventSizeTextVariantEnum::Variant1536x1024 => "1536x1024",
            ImageEditCompletedEventSizeTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for ImageEditCompletedEventSizeTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "1024x1024" => Ok(Self::Variant1024x1024),
            "1024x1536" => Ok(Self::Variant1024x1536),
            "1536x1024" => Ok(Self::Variant1536x1024),
            "auto" => Ok(Self::Auto),
            _ => Err(crate::models::ParseEnumError::new(
                "ImageEditCompletedEventSizeTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for ImageEditCompletedEventSizeTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for ImageEditCompletedEventSizeTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for ImageEditCompletedEventSizeTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
