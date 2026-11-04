use serde::{Deserialize, Serialize};

/// RealtimeBetaResponseCreateParamsConversation - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeBetaResponseCreateParamsConversation {
    Text(String),
    TextVariant(RealtimeBetaResponseCreateParamsConversationTextVariantEnum),
}

impl RealtimeBetaResponseCreateParamsConversation {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for RealtimeBetaResponseCreateParamsConversation {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for RealtimeBetaResponseCreateParamsConversation {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for RealtimeBetaResponseCreateParamsConversation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RealtimeBetaResponseCreateParamsConversation::Text(value) => write!(f, "{}", value),
            RealtimeBetaResponseCreateParamsConversation::TextVariant(value) => {
                write!(f, "{}", value)
            }
        }
    }
}

/// RealtimeBetaResponseCreateParamsConversationTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeBetaResponseCreateParamsConversationTextVariantEnum {
    Auto,
    None,
}

impl std::fmt::Display for RealtimeBetaResponseCreateParamsConversationTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RealtimeBetaResponseCreateParamsConversationTextVariantEnum::Auto => "auto",
            RealtimeBetaResponseCreateParamsConversationTextVariantEnum::None => "none",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for RealtimeBetaResponseCreateParamsConversationTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "auto" => Ok(Self::Auto),
            "none" => Ok(Self::None),
            _ => Err(crate::models::ParseEnumError::new(
                "RealtimeBetaResponseCreateParamsConversationTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for RealtimeBetaResponseCreateParamsConversationTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for RealtimeBetaResponseCreateParamsConversationTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for RealtimeBetaResponseCreateParamsConversationTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
