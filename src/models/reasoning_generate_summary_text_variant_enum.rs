use serde::{Deserialize, Serialize};

/// ReasoningGenerateSummaryTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningGenerateSummaryTextVariantEnum {
    Auto,
    Concise,
    Detailed,
}

impl std::fmt::Display for ReasoningGenerateSummaryTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ReasoningGenerateSummaryTextVariantEnum::Auto => "auto",
            ReasoningGenerateSummaryTextVariantEnum::Concise => "concise",
            ReasoningGenerateSummaryTextVariantEnum::Detailed => "detailed",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for ReasoningGenerateSummaryTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "auto" => Ok(Self::Auto),
            "concise" => Ok(Self::Concise),
            "detailed" => Ok(Self::Detailed),
            _ => Err(crate::models::ParseEnumError::new(
                "ReasoningGenerateSummaryTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for ReasoningGenerateSummaryTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for ReasoningGenerateSummaryTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for ReasoningGenerateSummaryTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
