use crate::models;
use serde::{Deserialize, Serialize};

/// FineTuneReinforcementHyperparametersComputeMultiplier - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneReinforcementHyperparametersComputeMultiplier {
    TextVariant(FineTuneReinforcementHyperparametersComputeMultiplierTextVariantEnum),
}

impl std::fmt::Display for FineTuneReinforcementHyperparametersComputeMultiplier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FineTuneReinforcementHyperparametersComputeMultiplier::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

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
