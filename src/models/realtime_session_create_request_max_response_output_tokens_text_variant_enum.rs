use serde::{Deserialize, Serialize};

/// RealtimeSessionCreateRequestMaxResponseOutputTokensTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeSessionCreateRequestMaxResponseOutputTokensTextVariantEnum {
    Inf,
}

impl Default for RealtimeSessionCreateRequestMaxResponseOutputTokensTextVariantEnum {
    fn default() -> Self {
        Self::Inf
    }
}

impl std::fmt::Display for RealtimeSessionCreateRequestMaxResponseOutputTokensTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RealtimeSessionCreateRequestMaxResponseOutputTokensTextVariantEnum::Inf => "inf",
        };
        write!(f, "{}", value)
    }
}
