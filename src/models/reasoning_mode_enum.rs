use serde::{Deserialize, Serialize};

/// ReasoningModeEnum - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReasoningModeEnum {
    Text(String),
    TextVariant(ReasoningModeEnumTextVariantEnum),
}

impl ReasoningModeEnum {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for ReasoningModeEnum {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for ReasoningModeEnum {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for ReasoningModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReasoningModeEnum::Text(value) => write!(f, "{}", value),
            ReasoningModeEnum::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

/// ReasoningModeEnumTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningModeEnumTextVariantEnum {
    Standard,
    Pro,
}

impl std::fmt::Display for ReasoningModeEnumTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ReasoningModeEnumTextVariantEnum::Standard => "standard",
            ReasoningModeEnumTextVariantEnum::Pro => "pro",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for ReasoningModeEnumTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "standard" => Ok(Self::Standard),
            "pro" => Ok(Self::Pro),
            _ => Err(crate::models::ParseEnumError::new(
                "ReasoningModeEnumTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for ReasoningModeEnumTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for ReasoningModeEnumTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for ReasoningModeEnumTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
