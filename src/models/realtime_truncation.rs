use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeTruncation - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeTruncation {
    RealtimeTruncationStrategy(RealtimeTruncationStrategyEnum),
    Retention ratio truncation(models::ChatCompletionNamedToolChoice),
}

impl Default for RealtimeTruncation {
    fn default() -> Self {
        Self::RealtimeTruncationStrategy(Default::default())
    }
}

