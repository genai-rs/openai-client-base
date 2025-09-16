use serde::{Deserialize, Serialize};

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
