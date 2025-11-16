use crate::models;
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
}

impl Default for ReasoningEffortTextVariantEnum {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for ReasoningEffortTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ReasoningEffortTextVariantEnum::None => "none",
            ReasoningEffortTextVariantEnum::Minimal => "minimal",
            ReasoningEffortTextVariantEnum::Low => "low",
            ReasoningEffortTextVariantEnum::Medium => "medium",
            ReasoningEffortTextVariantEnum::High => "high",
        };
        write!(f, "{}", value)
    }
}
