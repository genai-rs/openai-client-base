use serde::{Deserialize, Serialize};

/// FineTuneReinforcementHyperparametersLearningRateMultiplierTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FineTuneReinforcementHyperparametersLearningRateMultiplierTextVariantEnum {
    Auto,
}

impl Default for FineTuneReinforcementHyperparametersLearningRateMultiplierTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for FineTuneReinforcementHyperparametersLearningRateMultiplierTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            FineTuneReinforcementHyperparametersLearningRateMultiplierTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
