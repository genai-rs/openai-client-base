use serde::{Deserialize, Serialize};

/// FineTuneSupervisedHyperparametersLearningRateMultiplierTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FineTuneSupervisedHyperparametersLearningRateMultiplierTextVariantEnum {
    Auto,
}

impl Default for FineTuneSupervisedHyperparametersLearningRateMultiplierTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for FineTuneSupervisedHyperparametersLearningRateMultiplierTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            FineTuneSupervisedHyperparametersLearningRateMultiplierTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
