use serde::{Deserialize, Serialize};

/// FunctionCallOutputItemParamStatusTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FunctionCallOutputItemParamStatusTextVariantEnum {
    #[serde(rename = "in_progress")]
    InProgress,
    Completed,
    Incomplete,
}

impl Default for FunctionCallOutputItemParamStatusTextVariantEnum {
    fn default() -> Self {
        Self::InProgress
    }
}

impl std::fmt::Display for FunctionCallOutputItemParamStatusTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            FunctionCallOutputItemParamStatusTextVariantEnum::InProgress => "in_progress",
            FunctionCallOutputItemParamStatusTextVariantEnum::Completed => "completed",
            FunctionCallOutputItemParamStatusTextVariantEnum::Incomplete => "incomplete",
        };
        write!(f, "{}", value)
    }
}
