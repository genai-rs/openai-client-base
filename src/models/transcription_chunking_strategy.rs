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

impl std::fmt::Display for TranscriptionChunkingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TranscriptionChunkingStrategy::TextVariant(v) => match v {
                TextVariantEnum::ItemReference => write!(f, "item_reference"),
            },
            TranscriptionChunkingStrategy::Vadconfig(cfg) => match serde_json::to_string(cfg) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}
