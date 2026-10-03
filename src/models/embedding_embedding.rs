use crate::models;
use serde::{Deserialize, Serialize};

/// EmbeddingEmbedding - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmbeddingEmbedding {
    Arrayofnumbers(Vec<serde_json::Value>),
    Text(String),
}

impl std::fmt::Display for EmbeddingEmbedding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmbeddingEmbedding::Arrayofnumbers(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            EmbeddingEmbedding::Text(value) => write!(f, "{}", value),
        }
    }
}
