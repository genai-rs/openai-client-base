use crate::models;
use serde::{Deserialize, Serialize};

/// CreateAssistantRequestToolResources - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAssistantRequestToolResources {
    Null,
}

impl Default for CreateAssistantRequestToolResources {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for CreateAssistantRequestToolResources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateAssistantRequestToolResources::Null => write!(f, "null"),
        }
    }
}

