use serde::{Deserialize, Serialize};

/// BetaLocalShellToolCallOutputStatusTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaLocalShellToolCallOutputStatusTextVariantEnum {
    #[serde(rename = "in_progress")]
    InProgress,
    Completed,
    Incomplete,
}

impl Default for BetaLocalShellToolCallOutputStatusTextVariantEnum {
    fn default() -> Self {
        Self::InProgress
    }
}

impl std::fmt::Display for BetaLocalShellToolCallOutputStatusTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaLocalShellToolCallOutputStatusTextVariantEnum::InProgress => "in_progress",
            BetaLocalShellToolCallOutputStatusTextVariantEnum::Completed => "completed",
            BetaLocalShellToolCallOutputStatusTextVariantEnum::Incomplete => "incomplete",
        };
        write!(f, "{}", value)
    }
}
