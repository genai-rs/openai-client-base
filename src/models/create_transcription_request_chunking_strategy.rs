use crate::models;
use serde::{Deserialize, Serialize};

/// CreateTranscriptionRequestChunkingStrategy - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTranscriptionRequestChunkingStrategy {
    TextVariant(CreateTranscriptionRequestChunkingStrategyTextVariantEnum),
    Vadconfig(models::VadConfig),
    Null,
}

impl std::fmt::Display for CreateTranscriptionRequestChunkingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateTranscriptionRequestChunkingStrategy::TextVariant(value) => {
                write!(f, "{}", value)
            }
            CreateTranscriptionRequestChunkingStrategy::Vadconfig(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            CreateTranscriptionRequestChunkingStrategy::Null => write!(f, "null"),
        }
    }
}

/// CreateTranscriptionRequestChunkingStrategyTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreateTranscriptionRequestChunkingStrategyTextVariantEnum {
    Auto,
}

impl Default for CreateTranscriptionRequestChunkingStrategyTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for CreateTranscriptionRequestChunkingStrategyTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            CreateTranscriptionRequestChunkingStrategyTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
