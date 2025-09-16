use serde::{Deserialize, Serialize};

/// McpToolRequireApprovalMcpToolApprovalSettingEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum McpToolRequireApprovalMcpToolApprovalSettingEnum {
    Always,
    Never,
}

impl Default for McpToolRequireApprovalMcpToolApprovalSettingEnum {
    fn default() -> Self {
        Self::Always
    }
}

impl std::fmt::Display for McpToolRequireApprovalMcpToolApprovalSettingEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            McpToolRequireApprovalMcpToolApprovalSettingEnum::Always => "always",
            McpToolRequireApprovalMcpToolApprovalSettingEnum::Never => "never",
        };
        write!(f, "{}", value)
    }
}
