use crate::models;
use serde::{Deserialize, Serialize};

/// FineTuneDpoHyperparametersNEpochs - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneDpoHyperparametersNEpochs {
    TextVariant(FineTuneDpoHyperparametersNEpochsTextVariantEnum),
}

impl std::fmt::Display for FineTuneDpoHyperparametersNEpochs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FineTuneDpoHyperparametersNEpochs::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

/// FineTuneDpoHyperparametersNEpochsTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FineTuneDpoHyperparametersNEpochsTextVariantEnum {
    Auto,
}

impl Default for FineTuneDpoHyperparametersNEpochsTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for FineTuneDpoHyperparametersNEpochsTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            FineTuneDpoHyperparametersNEpochsTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
