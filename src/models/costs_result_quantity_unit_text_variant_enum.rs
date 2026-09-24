use serde::{Deserialize, Serialize};

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
