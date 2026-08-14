use serde::{Deserialize, Serialize};

/// BetaReasoningContextTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaReasoningContextTextVariantEnum {
    Auto,
    #[serde(rename = "current_turn")]
    CurrentTurn,
    #[serde(rename = "all_turns")]
    AllTurns,
}

impl Default for BetaReasoningContextTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for BetaReasoningContextTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaReasoningContextTextVariantEnum::Auto => "auto",
            BetaReasoningContextTextVariantEnum::CurrentTurn => "current_turn",
            BetaReasoningContextTextVariantEnum::AllTurns => "all_turns",
        };
        write!(f, "{}", value)
    }
}
