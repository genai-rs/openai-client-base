use serde::{Deserialize, Serialize};

/// SpendLimitCurrencyTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SpendLimitCurrencyTextVariantEnum {
    #[serde(rename = "USD")]
    Usd,
}

impl Default for SpendLimitCurrencyTextVariantEnum {
    fn default() -> Self {
        Self::Usd
    }
}

impl std::fmt::Display for SpendLimitCurrencyTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            SpendLimitCurrencyTextVariantEnum::Usd => "USD",
        };
        write!(f, "{}", value)
    }
}
