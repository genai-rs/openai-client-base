use serde::{Deserialize, Serialize};

/// ModelIdsLive - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModelIdsLive {
    Text(String),
    TextVariant(ModelIdsLiveTextVariantEnum),
}

impl ModelIdsLive {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for ModelIdsLive {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for ModelIdsLive {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for ModelIdsLive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelIdsLive::Text(value) => write!(f, "{}", value),
            ModelIdsLive::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

/// ModelIdsLiveTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModelIdsLiveTextVariantEnum {
    #[serde(rename = "gpt-live-1")]
    GptLive1,
}

impl std::fmt::Display for ModelIdsLiveTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ModelIdsLiveTextVariantEnum::GptLive1 => "gpt-live-1",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for ModelIdsLiveTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "gpt-live-1" => Ok(Self::GptLive1),
            _ => Err(crate::models::ParseEnumError::new(
                "ModelIdsLiveTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for ModelIdsLiveTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for ModelIdsLiveTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for ModelIdsLiveTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
