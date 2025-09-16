use crate::models;
use serde::{Deserialize, Serialize};

/// CreateThreadRequestToolResources - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateThreadRequestToolResources {
    Null,
}

impl Default for CreateThreadRequestToolResources {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for CreateThreadRequestToolResources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateThreadRequestToolResources::Null => write!(f, "null"),
        }
    }
}
