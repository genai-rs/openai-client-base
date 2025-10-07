use crate::models;
use serde::{Deserialize, Serialize};

/// VectorStoreSearchRequestQuery - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VectorStoreSearchRequestQuery {
    Text(String),
    ArrayOfStrings(Vec<String>),
}

impl Default for VectorStoreSearchRequestQuery {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl VectorStoreSearchRequestQuery {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_arrayofstrings(items: Vec<String>) -> Self {
        Self::ArrayOfStrings(items)
    }
}

impl From<String> for VectorStoreSearchRequestQuery {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for VectorStoreSearchRequestQuery {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for VectorStoreSearchRequestQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VectorStoreSearchRequestQuery::Text(value) => write!(f, "{}", value),
            VectorStoreSearchRequestQuery::ArrayOfStrings(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
