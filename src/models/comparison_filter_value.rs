use crate::models;
use serde::{Deserialize, Serialize};

/// ComparisonFilterValue - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComparisonFilterValue {
    Text(String),
    Array(Vec<serde_json::Value>),
}

impl Default for ComparisonFilterValue {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl ComparisonFilterValue {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_array(items: Vec<serde_json::Value>) -> Self {
        Self::Array(items)
    }
}

impl From<String> for ComparisonFilterValue {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for ComparisonFilterValue {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for ComparisonFilterValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComparisonFilterValue::Text(value) => write!(f, "{}", value),
            ComparisonFilterValue::Array(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}
