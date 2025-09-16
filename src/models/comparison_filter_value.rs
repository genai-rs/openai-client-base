use crate::models;
use serde::{Deserialize, Serialize};

/// ComparisonFilterValue - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComparisonFilterValue {
    Text(String),
}

impl Default for ComparisonFilterValue {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl std::fmt::Display for ComparisonFilterValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComparisonFilterValue::Text(value) => write!(f, "{}", value),
        }
    }
}
