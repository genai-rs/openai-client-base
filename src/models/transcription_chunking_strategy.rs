use crate::models;
use serde::{Deserialize, Serialize};

/// TranscriptionChunkingStrategy - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TranscriptionChunkingStrategy {
    TextVariant(TranscriptionChunkingStrategyTextVariantEnum),
    Vadconfig(models::VadConfig),
    Null,
}

impl std::fmt::Display for TranscriptionChunkingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TranscriptionChunkingStrategy::TextVariant(value) => write!(f, "{}", value),
            TranscriptionChunkingStrategy::Vadconfig(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            TranscriptionChunkingStrategy::Null => write!(f, "null"),
        }
    }
}

/// TranscriptionChunkingStrategyTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TranscriptionChunkingStrategyTextVariantEnum {
    Auto,
}

impl Default for TranscriptionChunkingStrategyTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for TranscriptionChunkingStrategyTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            TranscriptionChunkingStrategyTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
