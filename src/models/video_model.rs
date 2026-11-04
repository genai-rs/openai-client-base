use serde::{Deserialize, Serialize};

/// VideoModel - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VideoModel {
    Text(String),
    TextVariant(VideoModelTextVariantEnum),
}

impl VideoModel {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for VideoModel {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for VideoModel {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for VideoModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoModel::Text(value) => write!(f, "{}", value),
            VideoModel::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

/// VideoModelTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoModelTextVariantEnum {
    #[serde(rename = "sora-2")]
    Sora2,
    #[serde(rename = "sora-2-pro")]
    Sora2Pro,
    #[serde(rename = "sora-2-2025-10-06")]
    Sora220251006,
    #[serde(rename = "sora-2-pro-2025-10-06")]
    Sora2Pro20251006,
    #[serde(rename = "sora-2-2025-12-08")]
    Sora220251208,
}

impl std::fmt::Display for VideoModelTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            VideoModelTextVariantEnum::Sora2 => "sora-2",
            VideoModelTextVariantEnum::Sora2Pro => "sora-2-pro",
            VideoModelTextVariantEnum::Sora220251006 => "sora-2-2025-10-06",
            VideoModelTextVariantEnum::Sora2Pro20251006 => "sora-2-pro-2025-10-06",
            VideoModelTextVariantEnum::Sora220251208 => "sora-2-2025-12-08",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for VideoModelTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "sora-2" => Ok(Self::Sora2),
            "sora-2-pro" => Ok(Self::Sora2Pro),
            "sora-2-2025-10-06" => Ok(Self::Sora220251006),
            "sora-2-pro-2025-10-06" => Ok(Self::Sora2Pro20251006),
            "sora-2-2025-12-08" => Ok(Self::Sora220251208),
            _ => Err(crate::models::ParseEnumError::new(
                "VideoModelTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for VideoModelTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for VideoModelTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for VideoModelTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
