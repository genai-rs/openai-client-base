use serde::{Deserialize, Serialize};

/// RealtimeSessionCreateRequestGaMaxOutputTokensTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeSessionCreateRequestGaMaxOutputTokensTextVariantEnum {
    Inf,
}

impl Default for RealtimeSessionCreateRequestGaMaxOutputTokensTextVariantEnum {
    fn default() -> Self {
        Self::Inf
    }
}

impl std::fmt::Display for RealtimeSessionCreateRequestGaMaxOutputTokensTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RealtimeSessionCreateRequestGaMaxOutputTokensTextVariantEnum::Inf => "inf",
        };
        write!(f, "{}", value)
    }
}
