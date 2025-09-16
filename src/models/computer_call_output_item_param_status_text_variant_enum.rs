use serde::{Deserialize, Serialize};

/// ComputerCallOutputItemParamStatusTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ComputerCallOutputItemParamStatusTextVariantEnum {
    #[serde(rename = "in_progress")]
    InProgress,
    Completed,
    Incomplete,
}

impl Default for ComputerCallOutputItemParamStatusTextVariantEnum {
    fn default() -> Self {
        Self::InProgress
    }
}

impl std::fmt::Display for ComputerCallOutputItemParamStatusTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ComputerCallOutputItemParamStatusTextVariantEnum::InProgress => "in_progress",
            ComputerCallOutputItemParamStatusTextVariantEnum::Completed => "completed",
            ComputerCallOutputItemParamStatusTextVariantEnum::Incomplete => "incomplete",
        };
        write!(f, "{}", value)
    }
}
