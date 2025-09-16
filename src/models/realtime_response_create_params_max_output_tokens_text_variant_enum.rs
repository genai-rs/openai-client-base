use serde::{Deserialize, Serialize};

/// RealtimeResponseCreateParamsMaxOutputTokensTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeResponseCreateParamsMaxOutputTokensTextVariantEnum {
    Inf,
}

impl Default for RealtimeResponseCreateParamsMaxOutputTokensTextVariantEnum {
    fn default() -> Self {
        Self::Inf
    }
}

impl std::fmt::Display for RealtimeResponseCreateParamsMaxOutputTokensTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RealtimeResponseCreateParamsMaxOutputTokensTextVariantEnum::Inf => "inf",
        };
        write!(f, "{}", value)
    }
}
