use crate::models;
use serde::{Deserialize, Serialize};

/// FineTuneDpoHyperparametersBatchSize - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneDpoHyperparametersBatchSize {
    TextVariant(FineTuneDpoHyperparametersBatchSizeTextVariantEnum),
}

impl std::fmt::Display for FineTuneDpoHyperparametersBatchSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FineTuneDpoHyperparametersBatchSize::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

/// FineTuneDpoHyperparametersBatchSizeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FineTuneDpoHyperparametersBatchSizeTextVariantEnum {
    Auto,
}

impl Default for FineTuneDpoHyperparametersBatchSizeTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for FineTuneDpoHyperparametersBatchSizeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            FineTuneDpoHyperparametersBatchSizeTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
