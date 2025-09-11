use crate::models;
use serde::{Deserialize, Serialize};

/// Filters - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Filters {
    Comparisonfilter(models::ComparisonFilter),
    Compoundfilter(models::CompoundFilter),
}

impl Default for Filters {
    fn default() -> Self {
        Self::Comparisonfilter(Default::default())
    }
}

