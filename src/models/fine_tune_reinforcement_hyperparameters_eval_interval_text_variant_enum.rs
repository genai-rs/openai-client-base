use serde::{Deserialize, Serialize};

/// FineTuneReinforcementHyperparametersEvalIntervalTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FineTuneReinforcementHyperparametersEvalIntervalTextVariantEnum {
    Auto,
}

impl Default for FineTuneReinforcementHyperparametersEvalIntervalTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for FineTuneReinforcementHyperparametersEvalIntervalTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            FineTuneReinforcementHyperparametersEvalIntervalTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
