use serde::{Deserialize, Serialize};

/// SpendLimitEnforcementStatusTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SpendLimitEnforcementStatusTextVariantEnum {
    Inactive,
    Enforcing,
}

impl std::fmt::Display for SpendLimitEnforcementStatusTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            SpendLimitEnforcementStatusTextVariantEnum::Inactive => "inactive",
            SpendLimitEnforcementStatusTextVariantEnum::Enforcing => "enforcing",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for SpendLimitEnforcementStatusTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "inactive" => Ok(Self::Inactive),
            "enforcing" => Ok(Self::Enforcing),
            _ => Err(crate::models::ParseEnumError::new(
                "SpendLimitEnforcementStatusTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for SpendLimitEnforcementStatusTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for SpendLimitEnforcementStatusTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for SpendLimitEnforcementStatusTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
