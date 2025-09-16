use serde::{Deserialize, Serialize};

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
