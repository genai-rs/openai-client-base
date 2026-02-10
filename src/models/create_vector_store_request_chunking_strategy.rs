use crate::models;
use serde::{Deserialize, Serialize};

/// CreateVectorStoreRequestChunkingStrategy - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateVectorStoreRequestChunkingStrategy {
    Autochunkingstrategyrequestparam(models::AutoChunkingStrategyRequestParam),
    Staticchunkingstrategyrequestparam(models::StaticChunkingStrategyRequestParam),
}

impl std::fmt::Display for CreateVectorStoreRequestChunkingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateVectorStoreRequestChunkingStrategy::Autochunkingstrategyrequestparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            CreateVectorStoreRequestChunkingStrategy::Staticchunkingstrategyrequestparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
