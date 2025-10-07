use crate::models;
use serde::{Deserialize, Serialize};

/// FineTuneReinforcementHyperparametersEvalInterval - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneReinforcementHyperparametersEvalInterval {
    TextVariant(FineTuneReinforcementHyperparametersEvalIntervalTextVariantEnum),
}

impl std::fmt::Display for FineTuneReinforcementHyperparametersEvalInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FineTuneReinforcementHyperparametersEvalInterval::TextVariant(value) => {
                write!(f, "{}", value)
            }
        }
    }
}

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
