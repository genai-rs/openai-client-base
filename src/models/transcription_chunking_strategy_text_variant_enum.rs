use serde::{Deserialize, Serialize};

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
