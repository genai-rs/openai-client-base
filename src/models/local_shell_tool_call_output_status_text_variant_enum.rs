use serde::{Deserialize, Serialize};

/// LocalShellToolCallOutputStatusTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LocalShellToolCallOutputStatusTextVariantEnum {
    #[serde(rename = "in_progress")]
    InProgress,
    Completed,
    Incomplete,
}

impl Default for LocalShellToolCallOutputStatusTextVariantEnum {
    fn default() -> Self {
        Self::InProgress
    }
}

impl std::fmt::Display for LocalShellToolCallOutputStatusTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            LocalShellToolCallOutputStatusTextVariantEnum::InProgress => "in_progress",
            LocalShellToolCallOutputStatusTextVariantEnum::Completed => "completed",
            LocalShellToolCallOutputStatusTextVariantEnum::Incomplete => "incomplete",
        };
        write!(f, "{}", value)
    }
}
