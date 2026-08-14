use crate::models;
use serde::{Deserialize, Serialize};

/// BetaReasoningEffort - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaReasoningEffort {
    TextVariant(BetaReasoningEffortTextVariantEnum),
    Null,
}

impl std::fmt::Display for BetaReasoningEffort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaReasoningEffort::TextVariant(value) => write!(f, "{}", value),
            BetaReasoningEffort::Null => write!(f, "null"),
        }
    }
}

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

impl Default for BetaReasoningEffortTextVariantEnum {
    fn default() -> Self {
        Self::None
    }
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
