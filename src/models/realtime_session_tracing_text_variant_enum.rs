use serde::{Deserialize, Serialize};

/// RealtimeSessionTracingTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeSessionTracingTextVariantEnum {
    Auto,
}

impl Default for RealtimeSessionTracingTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for RealtimeSessionTracingTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RealtimeSessionTracingTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
