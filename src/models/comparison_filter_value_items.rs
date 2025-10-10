use crate::models;
use serde::{Deserialize, Serialize};

/// ComparisonFilterValueItems - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComparisonFilterValueItems {
    Text(String),
}

impl Default for ComparisonFilterValueItems {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl std::fmt::Display for ComparisonFilterValueItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComparisonFilterValueItems::Text(value) => write!(f, "{}", value),
        }
    }
}
