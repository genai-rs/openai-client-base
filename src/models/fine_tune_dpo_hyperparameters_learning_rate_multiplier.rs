use crate::models;
use serde::{Deserialize, Serialize};

/// FineTuneDpoHyperparametersLearningRateMultiplier - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneDpoHyperparametersLearningRateMultiplier {
    TextVariant(FineTuneDpoHyperparametersLearningRateMultiplierTextVariantEnum),
}

impl std::fmt::Display for FineTuneDpoHyperparametersLearningRateMultiplier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FineTuneDpoHyperparametersLearningRateMultiplier::TextVariant(value) => {
                write!(f, "{}", value)
            }
        }
    }
}

/// FineTuneDpoHyperparametersLearningRateMultiplierTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FineTuneDpoHyperparametersLearningRateMultiplierTextVariantEnum {
    Auto,
}

impl Default for FineTuneDpoHyperparametersLearningRateMultiplierTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for FineTuneDpoHyperparametersLearningRateMultiplierTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            FineTuneDpoHyperparametersLearningRateMultiplierTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
