use serde::{Deserialize, Serialize};

/// BetaReasoningGenerateSummaryTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaReasoningGenerateSummaryTextVariantEnum {
    Auto,
    Concise,
    Detailed,
}

impl Default for BetaReasoningGenerateSummaryTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for BetaReasoningGenerateSummaryTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaReasoningGenerateSummaryTextVariantEnum::Auto => "auto",
            BetaReasoningGenerateSummaryTextVariantEnum::Concise => "concise",
            BetaReasoningGenerateSummaryTextVariantEnum::Detailed => "detailed",
        };
        write!(f, "{}", value)
    }
}
