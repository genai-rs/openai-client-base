use serde::{Deserialize, Serialize};

/// FineTuneSupervisedHyperparametersBatchSizeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FineTuneSupervisedHyperparametersBatchSizeTextVariantEnum {
    Auto,
}

impl Default for FineTuneSupervisedHyperparametersBatchSizeTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for FineTuneSupervisedHyperparametersBatchSizeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            FineTuneSupervisedHyperparametersBatchSizeTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
