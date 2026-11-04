use serde::{Deserialize, Serialize};

/// ReasoningEffort - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReasoningEffort {
    TextVariant(ReasoningEffortTextVariantEnum),
    Null,
}

impl std::fmt::Display for ReasoningEffort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReasoningEffort::TextVariant(value) => write!(f, "{}", value),
            ReasoningEffort::Null => write!(f, "null"),
        }
    }
}

/// ReasoningEffortTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffortTextVariantEnum {
    None,
    Minimal,
    Low,
    Medium,
    High,
    Xhigh,
    Max,
}

impl std::fmt::Display for ReasoningEffortTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ReasoningEffortTextVariantEnum::None => "none",
            ReasoningEffortTextVariantEnum::Minimal => "minimal",
            ReasoningEffortTextVariantEnum::Low => "low",
            ReasoningEffortTextVariantEnum::Medium => "medium",
            ReasoningEffortTextVariantEnum::High => "high",
            ReasoningEffortTextVariantEnum::Xhigh => "xhigh",
            ReasoningEffortTextVariantEnum::Max => "max",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for ReasoningEffortTextVariantEnum {
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
                "ReasoningEffortTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for ReasoningEffortTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for ReasoningEffortTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for ReasoningEffortTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
