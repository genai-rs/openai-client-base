use serde::{Deserialize, Serialize};

/// FineTuneReinforcementHyperparametersNEpochsTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FineTuneReinforcementHyperparametersNEpochsTextVariantEnum {
    Auto,
}

impl Default for FineTuneReinforcementHyperparametersNEpochsTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for FineTuneReinforcementHyperparametersNEpochsTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            FineTuneReinforcementHyperparametersNEpochsTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
