use serde::{Deserialize, Serialize};

/// FineTuneReinforcementHyperparametersBatchSizeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FineTuneReinforcementHyperparametersBatchSizeTextVariantEnum {
    Auto,
}

impl Default for FineTuneReinforcementHyperparametersBatchSizeTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for FineTuneReinforcementHyperparametersBatchSizeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            FineTuneReinforcementHyperparametersBatchSizeTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
