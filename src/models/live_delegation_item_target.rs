use serde::{Deserialize, Serialize};

/// LiveDelegationItemTarget - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LiveDelegationItemTarget {
    TextVariant(LiveDelegationItemTargetTextVariantEnum),
}

impl std::fmt::Display for LiveDelegationItemTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiveDelegationItemTarget::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

/// LiveDelegationItemTargetTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LiveDelegationItemTargetTextVariantEnum {
    Decisions,
}

impl std::fmt::Display for LiveDelegationItemTargetTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            LiveDelegationItemTargetTextVariantEnum::Decisions => "decisions",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for LiveDelegationItemTargetTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "decisions" => Ok(Self::Decisions),
            _ => Err(crate::models::ParseEnumError::new(
                "LiveDelegationItemTargetTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for LiveDelegationItemTargetTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for LiveDelegationItemTargetTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for LiveDelegationItemTargetTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
