use crate::models;
use serde::{Deserialize, Serialize};

/// ModifyAssistantRequestToolResources - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModifyAssistantRequestToolResources {
    Null,
}

impl Default for ModifyAssistantRequestToolResources {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for ModifyAssistantRequestToolResources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModifyAssistantRequestToolResources::Null => write!(f, "null"),
        }
    }
}
