use serde::{Deserialize, Serialize};

/// BetaModelResponsePropertiesPromptCacheRetentionTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaModelResponsePropertiesPromptCacheRetentionTextVariantEnum {
    #[serde(rename = "in_memory")]
    InMemory,
    #[serde(rename = "24h")]
    Variant24h,
}

impl std::fmt::Display for BetaModelResponsePropertiesPromptCacheRetentionTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaModelResponsePropertiesPromptCacheRetentionTextVariantEnum::InMemory => "in_memory",
            BetaModelResponsePropertiesPromptCacheRetentionTextVariantEnum::Variant24h => "24h",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for BetaModelResponsePropertiesPromptCacheRetentionTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "in_memory" => Ok(Self::InMemory),
            "24h" => Ok(Self::Variant24h),
            _ => Err(crate::models::ParseEnumError::new(
                "BetaModelResponsePropertiesPromptCacheRetentionTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str>
    for BetaModelResponsePropertiesPromptCacheRetentionTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for BetaModelResponsePropertiesPromptCacheRetentionTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for BetaModelResponsePropertiesPromptCacheRetentionTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
