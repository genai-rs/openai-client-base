use serde::{Deserialize, Serialize};

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
