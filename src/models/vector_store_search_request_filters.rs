use crate::models;
use serde::{Deserialize, Serialize};

/// VectorStoreSearchRequestFilters - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VectorStoreSearchRequestFilters {
    ComparisonFilter(models::ComparisonFilter),
    CompoundFilter(models::CompoundFilter),
}

impl Default for VectorStoreSearchRequestFilters {
    fn default() -> Self {
        Self::ComparisonFilter(Default::default())
    }
}

