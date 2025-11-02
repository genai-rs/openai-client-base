use serde::{Deserialize, Serialize};

/// RealtimeSessionCreateRequestGaTracingAutoEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeSessionCreateRequestGaTracingAutoEnum {
    Auto,
}

impl Default for RealtimeSessionCreateRequestGaTracingAutoEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for RealtimeSessionCreateRequestGaTracingAutoEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RealtimeSessionCreateRequestGaTracingAutoEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
