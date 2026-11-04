use serde::{Deserialize, Serialize};

/// ChatCompletionToolChoiceOptionToolChoiceModeEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatCompletionToolChoiceOptionToolChoiceModeEnum {
    None,
    Auto,
    Required,
}

impl std::fmt::Display for ChatCompletionToolChoiceOptionToolChoiceModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ChatCompletionToolChoiceOptionToolChoiceModeEnum::None => "none",
            ChatCompletionToolChoiceOptionToolChoiceModeEnum::Auto => "auto",
            ChatCompletionToolChoiceOptionToolChoiceModeEnum::Required => "required",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for ChatCompletionToolChoiceOptionToolChoiceModeEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "none" => Ok(Self::None),
            "auto" => Ok(Self::Auto),
            "required" => Ok(Self::Required),
            _ => Err(crate::models::ParseEnumError::new(
                "ChatCompletionToolChoiceOptionToolChoiceModeEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for ChatCompletionToolChoiceOptionToolChoiceModeEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for ChatCompletionToolChoiceOptionToolChoiceModeEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for ChatCompletionToolChoiceOptionToolChoiceModeEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
