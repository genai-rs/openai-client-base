use serde::{Deserialize, Serialize};

/// ReasoningGenerateSummaryTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningGenerateSummaryTextVariantEnum {
    Auto,
    Concise,
    Detailed,
}

impl Default for ReasoningGenerateSummaryTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for ReasoningGenerateSummaryTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ReasoningGenerateSummaryTextVariantEnum::Auto => "auto",
            ReasoningGenerateSummaryTextVariantEnum::Concise => "concise",
            ReasoningGenerateSummaryTextVariantEnum::Detailed => "detailed",
        };
        write!(f, "{}", value)
    }
}
