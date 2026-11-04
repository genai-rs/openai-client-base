use serde::{Deserialize, Serialize};

/// LocalShellToolCallOutputStatusTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LocalShellToolCallOutputStatusTextVariantEnum {
    #[serde(rename = "in_progress")]
    InProgress,
    Completed,
    Incomplete,
}

impl std::fmt::Display for LocalShellToolCallOutputStatusTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            LocalShellToolCallOutputStatusTextVariantEnum::InProgress => "in_progress",
            LocalShellToolCallOutputStatusTextVariantEnum::Completed => "completed",
            LocalShellToolCallOutputStatusTextVariantEnum::Incomplete => "incomplete",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for LocalShellToolCallOutputStatusTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err(crate::models::ParseEnumError::new(
                "LocalShellToolCallOutputStatusTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for LocalShellToolCallOutputStatusTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for LocalShellToolCallOutputStatusTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for LocalShellToolCallOutputStatusTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
