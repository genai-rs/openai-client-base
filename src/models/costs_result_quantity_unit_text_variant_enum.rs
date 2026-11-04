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

impl ::std::str::FromStr for CostsResultQuantityUnitTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "tokens" => Ok(Self::Tokens),
            "1000_tokens" => Ok(Self::Variant1000Tokens),
            "duration_seconds" => Ok(Self::DurationSeconds),
            "duration_minutes" => Ok(Self::DurationMinutes),
            "duration_hours" => Ok(Self::DurationHours),
            "gibibyte_hours" => Ok(Self::GibibyteHours),
            "images" => Ok(Self::Images),
            "characters" => Ok(Self::Characters),
            _ => Err(crate::models::ParseEnumError::new(
                "CostsResultQuantityUnitTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for CostsResultQuantityUnitTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for CostsResultQuantityUnitTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for CostsResultQuantityUnitTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
