use serde::{Deserialize, Serialize};

/// McpToolRequireApprovalMcpToolApprovalSettingEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum McpToolRequireApprovalMcpToolApprovalSettingEnum {
    Always,
    Never,
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

impl ::std::str::FromStr for McpToolRequireApprovalMcpToolApprovalSettingEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            _ => Err(crate::models::ParseEnumError::new(
                "McpToolRequireApprovalMcpToolApprovalSettingEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for McpToolRequireApprovalMcpToolApprovalSettingEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for McpToolRequireApprovalMcpToolApprovalSettingEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for McpToolRequireApprovalMcpToolApprovalSettingEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
