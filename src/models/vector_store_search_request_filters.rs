use crate::models;
use serde::{Deserialize, Serialize};

/// VectorStoreSearchRequestFilters - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VectorStoreSearchRequestFilters {
    Comparisonfilter(models::ComparisonFilter),
    Compoundfilter(models::CompoundFilter),
}

impl Default for VectorStoreSearchRequestFilters {
    fn default() -> Self {
        Self::Comparisonfilter(Default::default())
    }
}

