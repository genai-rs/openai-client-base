use serde::{Deserialize, Serialize};

/// BetaMcpToolRequireApproval - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaMcpToolRequireApproval {
    McpToolApprovalSetting(BetaMcpToolRequireApprovalMcpToolApprovalSettingEnum),
    Null,
}

impl std::fmt::Display for BetaMcpToolRequireApproval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaMcpToolRequireApproval::McpToolApprovalSetting(value) => write!(f, "{}", value),
            BetaMcpToolRequireApproval::Null => write!(f, "null"),
        }
    }
}

/// BetaMcpToolRequireApprovalMcpToolApprovalSettingEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaMcpToolRequireApprovalMcpToolApprovalSettingEnum {
    Always,
    Never,
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

impl ::std::str::FromStr for BetaMcpToolRequireApprovalMcpToolApprovalSettingEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            _ => Err(crate::models::ParseEnumError::new(
                "BetaMcpToolRequireApprovalMcpToolApprovalSettingEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for BetaMcpToolRequireApprovalMcpToolApprovalSettingEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for BetaMcpToolRequireApprovalMcpToolApprovalSettingEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for BetaMcpToolRequireApprovalMcpToolApprovalSettingEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
