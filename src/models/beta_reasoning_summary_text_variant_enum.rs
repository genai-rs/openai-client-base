use serde::{Deserialize, Serialize};

/// BetaReasoningSummaryTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaReasoningSummaryTextVariantEnum {
    Auto,
    Concise,
    Detailed,
}

impl Default for BetaReasoningSummaryTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for BetaReasoningSummaryTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaReasoningSummaryTextVariantEnum::Auto => "auto",
            BetaReasoningSummaryTextVariantEnum::Concise => "concise",
            BetaReasoningSummaryTextVariantEnum::Detailed => "detailed",
        };
        write!(f, "{}", value)
    }
}
