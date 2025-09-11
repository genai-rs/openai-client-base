use crate::models;
use serde::{Deserialize, Serialize};

/// CreateEmbeddingRequestInput - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEmbeddingRequestInput {
    Text(String),
    ArrayOfStrings(Vec<String>),
    ArrayOfIntegers(Vec<i32>),
    ArrayOfIntegerArrays(Vec<Vec<i32>>),
}

impl Default for CreateEmbeddingRequestInput {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

