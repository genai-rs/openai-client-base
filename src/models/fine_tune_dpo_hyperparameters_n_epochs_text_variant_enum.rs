use serde::{Deserialize, Serialize};

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
