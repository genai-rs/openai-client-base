use crate::models;
use serde::{Deserialize, Serialize};

/// CreateEmbeddingRequestInput - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEmbeddingRequestInput {
    String(String),
    ArrayOfStrings(Vec<String>),
    ArrayOfIntegers(Vec<i32>),
    ArrayOfIntegerArrays(Vec<Vec<i32>>),
}

impl Default for CreateEmbeddingRequestInput {
    fn default() -> Self {
        Self::String(String::new())
    }
}

impl CreateEmbeddingRequestInput {
    pub fn new_text(text: String) -> Self {
        Self::String(text)
    }
    pub fn new_arrayofstrings(items: Vec<String>) -> Self {
        Self::ArrayOfStrings(items)
    }
    pub fn new_arrayofintegers(items: Vec<i32>) -> Self {
        Self::ArrayOfIntegers(items)
    }
    pub fn new_arrayofintegerarrays(items: Vec<Vec<i32>>) -> Self {
        Self::ArrayOfIntegerArrays(items)
    }
}

impl From<String> for CreateEmbeddingRequestInput {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for CreateEmbeddingRequestInput {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}
impl std::fmt::Display for CreateEmbeddingRequestInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateEmbeddingRequestInput::String(value) => write!(f, "{}", value),
            CreateEmbeddingRequestInput::ArrayOfStrings(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            CreateEmbeddingRequestInput::ArrayOfIntegers(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            CreateEmbeddingRequestInput::ArrayOfIntegerArrays(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
