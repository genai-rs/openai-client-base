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
