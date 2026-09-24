use crate::models;
use serde::{Deserialize, Serialize};

/// ResponseSteerPendingReason - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseSteerPendingReason {
    TextVariant(ResponseSteerPendingReasonTextVariantEnum),
    Text(String),
}

impl std::fmt::Display for ResponseSteerPendingReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseSteerPendingReason::TextVariant(value) => write!(f, "{}", value),
            ResponseSteerPendingReason::Text(value) => write!(f, "{}", value),
        }
    }
}

/// ResponseSteerPendingReasonTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponseSteerPendingReasonTextVariantEnum {
    #[serde(rename = "waiting_for_required_input")]
    WaitingForRequiredInput,
}

impl Default for ResponseSteerPendingReasonTextVariantEnum {
    fn default() -> Self {
        Self::WaitingForRequiredInput
    }
}

impl std::fmt::Display for ResponseSteerPendingReasonTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ResponseSteerPendingReasonTextVariantEnum::WaitingForRequiredInput => {
                "waiting_for_required_input"
            }
        };
        write!(f, "{}", value)
    }
}
