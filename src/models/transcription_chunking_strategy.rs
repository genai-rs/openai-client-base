use crate::models;
use serde::{Deserialize, Serialize};

/// TranscriptionChunkingStrategy - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TranscriptionChunkingStrategy {
    TextVariant(TextVariantEnum),
    Vadconfig(models::VadConfig),
}

/// TextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TextVariantEnum {
    #[serde(rename = "item_reference")]
    ItemReference,
}

impl Default for TextVariantEnum {
    fn default() -> Self {
        Self::ItemReference
    }
}
