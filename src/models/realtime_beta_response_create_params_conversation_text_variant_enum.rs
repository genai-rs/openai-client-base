use serde::{Deserialize, Serialize};

/// RealtimeBetaResponseCreateParamsConversationTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeBetaResponseCreateParamsConversationTextVariantEnum {
    Auto,
    None,
}

impl Default for RealtimeBetaResponseCreateParamsConversationTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for RealtimeBetaResponseCreateParamsConversationTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RealtimeBetaResponseCreateParamsConversationTextVariantEnum::Auto => "auto",
            RealtimeBetaResponseCreateParamsConversationTextVariantEnum::None => "none",
        };
        write!(f, "{}", value)
    }
}
