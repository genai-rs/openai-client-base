use serde::{Deserialize, Serialize};

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
