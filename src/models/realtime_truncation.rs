use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeTruncation - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeTruncation {
    Realtimetruncationstrategy(RealtimeTruncationRealtimetruncationstrategyEnum),
    RetentionRatioTruncation(models::ChatCompletionNamedToolChoice),
}

impl std::fmt::Display for RealtimeTruncation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RealtimeTruncation::Realtimetruncationstrategy(value) => write!(f, "{}", value),
            RealtimeTruncation::RetentionRatioTruncation(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}

/// RealtimeTruncationRealtimetruncationstrategyEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeTruncationRealtimetruncationstrategyEnum {
    Auto,
    Disabled,
}

impl Default for RealtimeTruncationRealtimetruncationstrategyEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for RealtimeTruncationRealtimetruncationstrategyEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RealtimeTruncationRealtimetruncationstrategyEnum::Auto => "auto",
            RealtimeTruncationRealtimetruncationstrategyEnum::Disabled => "disabled",
        };
        write!(f, "{}", value)
    }
}
