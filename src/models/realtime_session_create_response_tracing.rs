use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeSessionCreateResponseTracing - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeSessionCreateResponseTracing {
    TextVariant(RealtimeSessionCreateResponseTracingTextVariantEnum),
}

impl std::fmt::Display for RealtimeSessionCreateResponseTracing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RealtimeSessionCreateResponseTracing::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

/// RealtimeSessionCreateResponseTracingTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeSessionCreateResponseTracingTextVariantEnum {
    Auto,
}

impl Default for RealtimeSessionCreateResponseTracingTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for RealtimeSessionCreateResponseTracingTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RealtimeSessionCreateResponseTracingTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
