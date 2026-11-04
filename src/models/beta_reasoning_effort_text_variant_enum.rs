use serde::{Deserialize, Serialize};

/// BetaReasoningEffortTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaReasoningEffortTextVariantEnum {
    None,
    Minimal,
    Low,
    Medium,
    High,
    Xhigh,
    Max,
}

impl std::fmt::Display for BetaReasoningEffortTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaReasoningEffortTextVariantEnum::None => "none",
            BetaReasoningEffortTextVariantEnum::Minimal => "minimal",
            BetaReasoningEffortTextVariantEnum::Low => "low",
            BetaReasoningEffortTextVariantEnum::Medium => "medium",
            BetaReasoningEffortTextVariantEnum::High => "high",
            BetaReasoningEffortTextVariantEnum::Xhigh => "xhigh",
            BetaReasoningEffortTextVariantEnum::Max => "max",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for BetaReasoningEffortTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "none" => Ok(Self::None),
            "minimal" => Ok(Self::Minimal),
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "xhigh" => Ok(Self::Xhigh),
            "max" => Ok(Self::Max),
            _ => Err(crate::models::ParseEnumError::new(
                "BetaReasoningEffortTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for BetaReasoningEffortTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for BetaReasoningEffortTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for BetaReasoningEffortTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
