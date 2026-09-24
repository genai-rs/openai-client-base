use crate::models;
use serde::{Deserialize, Serialize};

/// CostsResultQuantityUnit - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CostsResultQuantityUnit {
    Text(String),
    TextVariant(CostsResultQuantityUnitTextVariantEnum),
    Null,
}

impl Default for CostsResultQuantityUnit {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl CostsResultQuantityUnit {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for CostsResultQuantityUnit {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for CostsResultQuantityUnit {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for CostsResultQuantityUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CostsResultQuantityUnit::Text(value) => write!(f, "{}", value),
            CostsResultQuantityUnit::TextVariant(value) => write!(f, "{}", value),
            CostsResultQuantityUnit::Null => write!(f, "null"),
        }
    }
}

/// CostsResultQuantityUnitTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CostsResultQuantityUnitTextVariantEnum {
    Tokens,
    #[serde(rename = "1000_tokens")]
    Variant1000Tokens,
    #[serde(rename = "duration_seconds")]
    DurationSeconds,
    #[serde(rename = "duration_minutes")]
    DurationMinutes,
    #[serde(rename = "duration_hours")]
    DurationHours,
    #[serde(rename = "gibibyte_hours")]
    GibibyteHours,
    Images,
    Characters,
}

impl Default for CostsResultQuantityUnitTextVariantEnum {
    fn default() -> Self {
        Self::Tokens
    }
}

impl std::fmt::Display for CostsResultQuantityUnitTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            CostsResultQuantityUnitTextVariantEnum::Tokens => "tokens",
            CostsResultQuantityUnitTextVariantEnum::Variant1000Tokens => "1000_tokens",
            CostsResultQuantityUnitTextVariantEnum::DurationSeconds => "duration_seconds",
            CostsResultQuantityUnitTextVariantEnum::DurationMinutes => "duration_minutes",
            CostsResultQuantityUnitTextVariantEnum::DurationHours => "duration_hours",
            CostsResultQuantityUnitTextVariantEnum::GibibyteHours => "gibibyte_hours",
            CostsResultQuantityUnitTextVariantEnum::Images => "images",
            CostsResultQuantityUnitTextVariantEnum::Characters => "characters",
        };
        write!(f, "{}", value)
    }
}
