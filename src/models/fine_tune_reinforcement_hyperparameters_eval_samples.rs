use crate::models;
use serde::{Deserialize, Serialize};

/// FineTuneReinforcementHyperparametersEvalSamples - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneReinforcementHyperparametersEvalSamples {
    TextVariant(FineTuneReinforcementHyperparametersEvalSamplesTextVariantEnum),
}

impl std::fmt::Display for FineTuneReinforcementHyperparametersEvalSamples {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FineTuneReinforcementHyperparametersEvalSamples::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

/// FineTuneReinforcementHyperparametersEvalSamplesTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FineTuneReinforcementHyperparametersEvalSamplesTextVariantEnum {
    Auto,
}

impl Default for FineTuneReinforcementHyperparametersEvalSamplesTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for FineTuneReinforcementHyperparametersEvalSamplesTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            FineTuneReinforcementHyperparametersEvalSamplesTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
