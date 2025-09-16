use serde::{Deserialize, Serialize};

/// RealtimeBetaResponseMaxOutputTokensTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeBetaResponseMaxOutputTokensTextVariantEnum {
    Inf,
}

impl Default for RealtimeBetaResponseMaxOutputTokensTextVariantEnum {
    fn default() -> Self {
        Self::Inf
    }
}

impl std::fmt::Display for RealtimeBetaResponseMaxOutputTokensTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RealtimeBetaResponseMaxOutputTokensTextVariantEnum::Inf => "inf",
        };
        write!(f, "{}", value)
    }
}
