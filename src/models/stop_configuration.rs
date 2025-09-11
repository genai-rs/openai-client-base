use crate::models;
use serde::{Deserialize, Serialize};

/// StopConfiguration - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StopConfiguration {
    Text(String),
    ArrayOfStrings(Vec<String>),
}

impl Default for StopConfiguration {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl StopConfiguration {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_arrayofstrings(items: Vec<String>) -> Self {
        Self::ArrayOfStrings(items)
    }
}

impl From<String> for StopConfiguration {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for StopConfiguration {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
