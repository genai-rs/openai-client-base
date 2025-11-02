use serde::{Deserialize, Serialize};

/// RealtimeSessionCreateResponseGaTracingAutoEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeSessionCreateResponseGaTracingAutoEnum {
    Auto,
}

impl Default for RealtimeSessionCreateResponseGaTracingAutoEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for RealtimeSessionCreateResponseGaTracingAutoEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RealtimeSessionCreateResponseGaTracingAutoEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
