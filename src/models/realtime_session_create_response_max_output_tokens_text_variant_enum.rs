use serde::{Deserialize, Serialize};

/// RealtimeSessionCreateResponseMaxOutputTokensTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeSessionCreateResponseMaxOutputTokensTextVariantEnum {
    Inf,
}

impl Default for RealtimeSessionCreateResponseMaxOutputTokensTextVariantEnum {
    fn default() -> Self {
        Self::Inf
    }
}

impl std::fmt::Display for RealtimeSessionCreateResponseMaxOutputTokensTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RealtimeSessionCreateResponseMaxOutputTokensTextVariantEnum::Inf => "inf",
        };
        write!(f, "{}", value)
    }
}
