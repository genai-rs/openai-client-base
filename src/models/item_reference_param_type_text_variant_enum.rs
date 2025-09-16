use serde::{Deserialize, Serialize};

/// ItemReferenceParamTypeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemReferenceParamTypeTextVariantEnum {
    #[serde(rename = "item_reference")]
    ItemReference,
}

impl Default for ItemReferenceParamTypeTextVariantEnum {
    fn default() -> Self {
        Self::ItemReference
    }
}

impl std::fmt::Display for ItemReferenceParamTypeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ItemReferenceParamTypeTextVariantEnum::ItemReference => "item_reference",
        };
        write!(f, "{}", value)
    }
}
