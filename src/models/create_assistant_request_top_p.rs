use crate::models;
use serde::{Deserialize, Serialize};

/// CreateAssistantRequestTopP - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAssistantRequestTopP {
    Null,
}

impl Default for CreateAssistantRequestTopP {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for CreateAssistantRequestTopP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateAssistantRequestTopP::Null => write!(f, "null"),
        }
    }
}
