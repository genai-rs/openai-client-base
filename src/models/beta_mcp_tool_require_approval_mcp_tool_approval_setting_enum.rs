use serde::{Deserialize, Serialize};

/// BetaMcpToolRequireApprovalMcpToolApprovalSettingEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaMcpToolRequireApprovalMcpToolApprovalSettingEnum {
    Always,
    Never,
}

impl Default for BetaMcpToolRequireApprovalMcpToolApprovalSettingEnum {
    fn default() -> Self {
        Self::Always
    }
}

impl std::fmt::Display for BetaMcpToolRequireApprovalMcpToolApprovalSettingEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaMcpToolRequireApprovalMcpToolApprovalSettingEnum::Always => "always",
            BetaMcpToolRequireApprovalMcpToolApprovalSettingEnum::Never => "never",
        };
        write!(f, "{}", value)
    }
}
