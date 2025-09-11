use crate::models;
use serde::{Deserialize, Serialize};

/// TranscriptionChunkingStrategy - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TranscriptionChunkingStrategy {
    Text(TextEnum),
    VadConfig(models::VadConfig),
}

impl Default for TranscriptionChunkingStrategy {
    fn default() -> Self {
        Self::Text(Default::default())
    }
}

