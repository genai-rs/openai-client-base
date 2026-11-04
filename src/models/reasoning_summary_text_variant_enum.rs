use serde::{Deserialize, Serialize};

/// ReasoningSummaryTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningSummaryTextVariantEnum {
    Auto,
    Concise,
    Detailed,
}

impl std::fmt::Display for ReasoningSummaryTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ReasoningSummaryTextVariantEnum::Auto => "auto",
            ReasoningSummaryTextVariantEnum::Concise => "concise",
            ReasoningSummaryTextVariantEnum::Detailed => "detailed",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for ReasoningSummaryTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "auto" => Ok(Self::Auto),
            "concise" => Ok(Self::Concise),
            "detailed" => Ok(Self::Detailed),
            _ => Err(crate::models::ParseEnumError::new(
                "ReasoningSummaryTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for ReasoningSummaryTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for ReasoningSummaryTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for ReasoningSummaryTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
