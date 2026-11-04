use serde::{Deserialize, Serialize};

/// ItemReferenceParamTypeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemReferenceParamTypeTextVariantEnum {
    #[serde(rename = "item_reference")]
    ItemReference,
}

impl std::fmt::Display for ItemReferenceParamTypeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ItemReferenceParamTypeTextVariantEnum::ItemReference => "item_reference",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for ItemReferenceParamTypeTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "item_reference" => Ok(Self::ItemReference),
            _ => Err(crate::models::ParseEnumError::new(
                "ItemReferenceParamTypeTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for ItemReferenceParamTypeTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for ItemReferenceParamTypeTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for ItemReferenceParamTypeTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
