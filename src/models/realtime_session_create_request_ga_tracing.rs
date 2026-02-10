use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeSessionCreateRequestGaTracing - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeSessionCreateRequestGaTracing {
    Auto(RealtimeSessionCreateRequestGaTracingAutoEnum),
}

impl std::fmt::Display for RealtimeSessionCreateRequestGaTracing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RealtimeSessionCreateRequestGaTracing::Auto(value) => write!(f, "{}", value),
        }
    }
}

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
