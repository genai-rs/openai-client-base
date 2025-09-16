use serde::{Deserialize, Serialize};

/// RealtimeSessionCreateResponseGaTracingTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeSessionCreateResponseGaTracingTextVariantEnum {
    Auto,
}

impl Default for RealtimeSessionCreateResponseGaTracingTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for RealtimeSessionCreateResponseGaTracingTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RealtimeSessionCreateResponseGaTracingTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
