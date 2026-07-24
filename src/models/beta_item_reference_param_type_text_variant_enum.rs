use serde::{Deserialize, Serialize};

/// BetaItemReferenceParamTypeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaItemReferenceParamTypeTextVariantEnum {
    #[serde(rename = "item_reference")]
    ItemReference,
}

impl Default for BetaItemReferenceParamTypeTextVariantEnum {
    fn default() -> Self {
        Self::ItemReference
    }
}

impl std::fmt::Display for BetaItemReferenceParamTypeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaItemReferenceParamTypeTextVariantEnum::ItemReference => "item_reference",
        };
        write!(f, "{}", value)
    }
}
