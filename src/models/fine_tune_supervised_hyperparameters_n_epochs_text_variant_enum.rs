use serde::{Deserialize, Serialize};

/// FineTuneSupervisedHyperparametersNEpochsTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FineTuneSupervisedHyperparametersNEpochsTextVariantEnum {
    Auto,
}

impl Default for FineTuneSupervisedHyperparametersNEpochsTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for FineTuneSupervisedHyperparametersNEpochsTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            FineTuneSupervisedHyperparametersNEpochsTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
