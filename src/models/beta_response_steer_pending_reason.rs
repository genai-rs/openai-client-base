use crate::models;
use serde::{Deserialize, Serialize};

/// BetaResponseSteerPendingReason - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaResponseSteerPendingReason {
    TextVariant(BetaResponseSteerPendingReasonTextVariantEnum),
    Text(String),
}

impl std::fmt::Display for BetaResponseSteerPendingReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaResponseSteerPendingReason::TextVariant(value) => write!(f, "{}", value),
            BetaResponseSteerPendingReason::Text(value) => write!(f, "{}", value),
        }
    }
}

/// BetaResponseSteerPendingReasonTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaResponseSteerPendingReasonTextVariantEnum {
    #[serde(rename = "waiting_for_required_input")]
    WaitingForRequiredInput,
}

impl Default for BetaResponseSteerPendingReasonTextVariantEnum {
    fn default() -> Self {
        Self::WaitingForRequiredInput
    }
}

impl std::fmt::Display for BetaResponseSteerPendingReasonTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaResponseSteerPendingReasonTextVariantEnum::WaitingForRequiredInput => {
                "waiting_for_required_input"
            }
        };
        write!(f, "{}", value)
    }
}
