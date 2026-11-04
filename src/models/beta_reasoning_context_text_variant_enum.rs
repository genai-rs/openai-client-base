use serde::{Deserialize, Serialize};

/// BetaReasoningContextTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaReasoningContextTextVariantEnum {
    Auto,
    #[serde(rename = "current_turn")]
    CurrentTurn,
    #[serde(rename = "all_turns")]
    AllTurns,
}

impl std::fmt::Display for BetaReasoningContextTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaReasoningContextTextVariantEnum::Auto => "auto",
            BetaReasoningContextTextVariantEnum::CurrentTurn => "current_turn",
            BetaReasoningContextTextVariantEnum::AllTurns => "all_turns",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for BetaReasoningContextTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "auto" => Ok(Self::Auto),
            "current_turn" => Ok(Self::CurrentTurn),
            "all_turns" => Ok(Self::AllTurns),
            _ => Err(crate::models::ParseEnumError::new(
                "BetaReasoningContextTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for BetaReasoningContextTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for BetaReasoningContextTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for BetaReasoningContextTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
