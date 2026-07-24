use crate::models;
use serde::{Deserialize, Serialize};

/// SpendLimitCurrency - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpendLimitCurrency {
    Text(String),
    TextVariant(SpendLimitCurrencyTextVariantEnum),
}

impl Default for SpendLimitCurrency {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl SpendLimitCurrency {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for SpendLimitCurrency {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for SpendLimitCurrency {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for SpendLimitCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpendLimitCurrency::Text(value) => write!(f, "{}", value),
            SpendLimitCurrency::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

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
