use serde::{Deserialize, Serialize};

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
            ReasoningEffortTextVariantEnum::Xhigh => "xhigh",
        };
        write!(f, "{}", value)
    }
}
