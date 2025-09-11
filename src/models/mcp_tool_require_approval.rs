use crate::models;
use serde::{Deserialize, Serialize};

/// MCPToolRequireApproval - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MCPToolRequireApproval {
    McpToolApprovalSetting(McpToolApprovalSettingEnum),
}

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
