use serde::{Deserialize, Serialize};

/// FineTuneReinforcementHyperparametersComputeMultiplierTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FineTuneReinforcementHyperparametersComputeMultiplierTextVariantEnum {
    Auto,
}

impl Default for FineTuneReinforcementHyperparametersComputeMultiplierTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for FineTuneReinforcementHyperparametersComputeMultiplierTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            FineTuneReinforcementHyperparametersComputeMultiplierTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
