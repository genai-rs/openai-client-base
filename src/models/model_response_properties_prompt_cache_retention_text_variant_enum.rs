use serde::{Deserialize, Serialize};

/// ModelResponsePropertiesPromptCacheRetentionTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModelResponsePropertiesPromptCacheRetentionTextVariantEnum {
    #[serde(rename = "in_memory")]
    InMemory,
    #[serde(rename = "24h")]
    Variant24h,
}

impl std::fmt::Display for ModelResponsePropertiesPromptCacheRetentionTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ModelResponsePropertiesPromptCacheRetentionTextVariantEnum::InMemory => "in_memory",
            ModelResponsePropertiesPromptCacheRetentionTextVariantEnum::Variant24h => "24h",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for ModelResponsePropertiesPromptCacheRetentionTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "in_memory" => Ok(Self::InMemory),
            "24h" => Ok(Self::Variant24h),
            _ => Err(crate::models::ParseEnumError::new(
                "ModelResponsePropertiesPromptCacheRetentionTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for ModelResponsePropertiesPromptCacheRetentionTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for ModelResponsePropertiesPromptCacheRetentionTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for ModelResponsePropertiesPromptCacheRetentionTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
