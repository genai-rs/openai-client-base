use crate::models;
use serde::{Deserialize, Serialize};

/// VectorStoreFileObjectChunkingStrategy - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VectorStoreFileObjectChunkingStrategy {
    Staticchunkingstrategyresponseparam(models::StaticChunkingStrategyResponseParam),
    Otherchunkingstrategyresponseparam(models::OtherChunkingStrategyResponseParam),
}

impl std::fmt::Display for VectorStoreFileObjectChunkingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VectorStoreFileObjectChunkingStrategy::Staticchunkingstrategyresponseparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            VectorStoreFileObjectChunkingStrategy::Otherchunkingstrategyresponseparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
