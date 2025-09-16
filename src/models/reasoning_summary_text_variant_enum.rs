use serde::{Deserialize, Serialize};

/// ReasoningSummaryTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningSummaryTextVariantEnum {
    Auto,
    Concise,
    Detailed,
}

impl Default for ReasoningSummaryTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for ReasoningSummaryTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ReasoningSummaryTextVariantEnum::Auto => "auto",
            ReasoningSummaryTextVariantEnum::Concise => "concise",
            ReasoningSummaryTextVariantEnum::Detailed => "detailed",
        };
        write!(f, "{}", value)
    }
}
