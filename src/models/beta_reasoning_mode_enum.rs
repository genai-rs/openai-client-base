use serde::{Deserialize, Serialize};

/// BetaReasoningModeEnum - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaReasoningModeEnum {
    Text(String),
    TextVariant(BetaReasoningModeEnumTextVariantEnum),
}

impl BetaReasoningModeEnum {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for BetaReasoningModeEnum {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for BetaReasoningModeEnum {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for BetaReasoningModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaReasoningModeEnum::Text(value) => write!(f, "{}", value),
            BetaReasoningModeEnum::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

/// BetaReasoningModeEnumTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaReasoningModeEnumTextVariantEnum {
    Standard,
    Pro,
}

impl std::fmt::Display for BetaReasoningModeEnumTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaReasoningModeEnumTextVariantEnum::Standard => "standard",
            BetaReasoningModeEnumTextVariantEnum::Pro => "pro",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for BetaReasoningModeEnumTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "standard" => Ok(Self::Standard),
            "pro" => Ok(Self::Pro),
            _ => Err(crate::models::ParseEnumError::new(
                "BetaReasoningModeEnumTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for BetaReasoningModeEnumTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for BetaReasoningModeEnumTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for BetaReasoningModeEnumTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
