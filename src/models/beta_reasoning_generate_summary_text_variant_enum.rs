use serde::{Deserialize, Serialize};

/// BetaReasoningGenerateSummaryTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaReasoningGenerateSummaryTextVariantEnum {
    Auto,
    Concise,
    Detailed,
}

impl std::fmt::Display for BetaReasoningGenerateSummaryTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaReasoningGenerateSummaryTextVariantEnum::Auto => "auto",
            BetaReasoningGenerateSummaryTextVariantEnum::Concise => "concise",
            BetaReasoningGenerateSummaryTextVariantEnum::Detailed => "detailed",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for BetaReasoningGenerateSummaryTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "auto" => Ok(Self::Auto),
            "concise" => Ok(Self::Concise),
            "detailed" => Ok(Self::Detailed),
            _ => Err(crate::models::ParseEnumError::new(
                "BetaReasoningGenerateSummaryTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for BetaReasoningGenerateSummaryTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for BetaReasoningGenerateSummaryTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for BetaReasoningGenerateSummaryTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
