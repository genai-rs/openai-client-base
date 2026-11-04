use serde::{Deserialize, Serialize};

/// SpendLimitInterval - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpendLimitInterval {
    Text(String),
    TextVariant(SpendLimitIntervalTextVariantEnum),
}

impl SpendLimitInterval {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for SpendLimitInterval {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for SpendLimitInterval {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for SpendLimitInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpendLimitInterval::Text(value) => write!(f, "{}", value),
            SpendLimitInterval::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

/// SpendLimitIntervalTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SpendLimitIntervalTextVariantEnum {
    Month,
}

impl std::fmt::Display for SpendLimitIntervalTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            SpendLimitIntervalTextVariantEnum::Month => "month",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for SpendLimitIntervalTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "month" => Ok(Self::Month),
            _ => Err(crate::models::ParseEnumError::new(
                "SpendLimitIntervalTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for SpendLimitIntervalTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for SpendLimitIntervalTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for SpendLimitIntervalTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
