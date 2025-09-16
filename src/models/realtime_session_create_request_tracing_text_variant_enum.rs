use serde::{Deserialize, Serialize};

/// RealtimeSessionCreateRequestTracingTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeSessionCreateRequestTracingTextVariantEnum {
    Auto,
}

impl Default for RealtimeSessionCreateRequestTracingTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for RealtimeSessionCreateRequestTracingTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RealtimeSessionCreateRequestTracingTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
