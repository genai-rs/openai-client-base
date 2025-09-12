use crate::models;
use serde::{Deserialize, Serialize};

/// Filters - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Filters {
    Comparisonfilter(models::ComparisonFilter),
    Compoundfilter(models::CompoundFilter),
}
