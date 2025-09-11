use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeTruncation - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeTruncation {
    Realtimetruncationstrategy(RealtimetruncationstrategyEnum),
    RetentionRatioTruncation(models::ChatCompletionNamedToolChoice),
}

/// RealtimetruncationstrategyEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimetruncationstrategyEnum {
    Auto,
    Disabled,
}

impl Default for RealtimetruncationstrategyEnum {
    fn default() -> Self {
        Self::Auto
    }
}
