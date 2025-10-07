use crate::models;
use serde::{Deserialize, Serialize};

/// WebSearchToolFilters - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebSearchToolFilters {
    Null,
}

impl Default for WebSearchToolFilters {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for WebSearchToolFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebSearchToolFilters::Null => write!(f, "null"),
        }
    }
}

