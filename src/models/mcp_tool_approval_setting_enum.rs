use serde::{Deserialize, Serialize};

/// McpToolApprovalSettingEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum McpToolApprovalSettingEnum {
    Always,
    Never,
}

impl Default for McpToolApprovalSettingEnum {
    fn default() -> Self {
        Self::Always
    }
}
