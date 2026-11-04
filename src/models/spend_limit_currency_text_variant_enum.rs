use serde::{Deserialize, Serialize};

/// SpendLimitCurrencyTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SpendLimitCurrencyTextVariantEnum {
    #[serde(rename = "USD")]
    Usd,
}

impl std::fmt::Display for SpendLimitCurrencyTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            SpendLimitCurrencyTextVariantEnum::Usd => "USD",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for SpendLimitCurrencyTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "USD" => Ok(Self::Usd),
            _ => Err(crate::models::ParseEnumError::new(
                "SpendLimitCurrencyTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for SpendLimitCurrencyTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for SpendLimitCurrencyTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for SpendLimitCurrencyTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
