use serde::{Deserialize, Serialize};

/// ReasoningContextTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningContextTextVariantEnum {
    Auto,
    #[serde(rename = "current_turn")]
    CurrentTurn,
    #[serde(rename = "all_turns")]
    AllTurns,
}

impl Default for ReasoningContextTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for ReasoningContextTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ReasoningContextTextVariantEnum::Auto => "auto",
            ReasoningContextTextVariantEnum::CurrentTurn => "current_turn",
            ReasoningContextTextVariantEnum::AllTurns => "all_turns",
        };
        write!(f, "{}", value)
    }
}
