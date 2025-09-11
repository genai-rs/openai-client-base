use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeTruncation - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeTruncation {
    Realtimetruncationstrategy(RealtimetruncationstrategyEnum),
    RetentionRatioTruncation(models::ChatCompletionNamedToolChoice),
}

impl Default for RealtimeTruncation {
    fn default() -> Self {
        Self::Realtimetruncationstrategy(Default::default())
    }
}

