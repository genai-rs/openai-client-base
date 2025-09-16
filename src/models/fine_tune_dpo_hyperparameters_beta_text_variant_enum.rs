use serde::{Deserialize, Serialize};

/// FineTuneDpoHyperparametersBetaTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FineTuneDpoHyperparametersBetaTextVariantEnum {
    Auto,
}

impl Default for FineTuneDpoHyperparametersBetaTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for FineTuneDpoHyperparametersBetaTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            FineTuneDpoHyperparametersBetaTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
