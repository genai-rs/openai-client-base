use serde::{Deserialize, Serialize};

/// RealtimeSessionCreateRequestGaTracingTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeSessionCreateRequestGaTracingTextVariantEnum {
    Auto,
}

impl Default for RealtimeSessionCreateRequestGaTracingTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for RealtimeSessionCreateRequestGaTracingTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RealtimeSessionCreateRequestGaTracingTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
