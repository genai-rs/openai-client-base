use serde::{Deserialize, Serialize};

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
