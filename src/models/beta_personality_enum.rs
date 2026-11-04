use serde::{Deserialize, Serialize};

/// BetaPersonalityEnum - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaPersonalityEnum {
    Text(String),
    TextVariant(BetaPersonalityEnumTextVariantEnum),
}

impl BetaPersonalityEnum {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for BetaPersonalityEnum {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for BetaPersonalityEnum {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for BetaPersonalityEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaPersonalityEnum::Text(value) => write!(f, "{}", value),
            BetaPersonalityEnum::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

/// BetaPersonalityEnumTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaPersonalityEnumTextVariantEnum {
    Friendly,
    Pragmatic,
}

impl std::fmt::Display for BetaPersonalityEnumTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaPersonalityEnumTextVariantEnum::Friendly => "friendly",
            BetaPersonalityEnumTextVariantEnum::Pragmatic => "pragmatic",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for BetaPersonalityEnumTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "friendly" => Ok(Self::Friendly),
            "pragmatic" => Ok(Self::Pragmatic),
            _ => Err(crate::models::ParseEnumError::new(
                "BetaPersonalityEnumTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for BetaPersonalityEnumTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for BetaPersonalityEnumTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for BetaPersonalityEnumTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
