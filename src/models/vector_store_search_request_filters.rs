use crate::models;
use serde::{Deserialize, Serialize};

/// VectorStoreSearchRequestFilters - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VectorStoreSearchRequestFilters {
    Comparisonfilter(models::ComparisonFilter),
    Compoundfilter(models::CompoundFilter),
}

impl std::fmt::Display for VectorStoreSearchRequestFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VectorStoreSearchRequestFilters::Comparisonfilter(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            VectorStoreSearchRequestFilters::Compoundfilter(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}

