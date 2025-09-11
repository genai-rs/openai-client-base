use serde::{Deserialize, Serialize};

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
