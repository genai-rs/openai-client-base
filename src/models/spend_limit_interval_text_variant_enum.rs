use serde::{Deserialize, Serialize};

/// SpendLimitIntervalTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SpendLimitIntervalTextVariantEnum {
    Month,
}

impl Default for SpendLimitIntervalTextVariantEnum {
    fn default() -> Self {
        Self::Month
    }
}

impl std::fmt::Display for SpendLimitIntervalTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            SpendLimitIntervalTextVariantEnum::Month => "month",
        };
        write!(f, "{}", value)
    }
}
