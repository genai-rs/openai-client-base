use crate::models;
use serde::{Deserialize, Serialize};

/// AssistantObjectToolResources - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssistantObjectToolResources {
    Null,
}

impl Default for AssistantObjectToolResources {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for AssistantObjectToolResources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssistantObjectToolResources::Null => write!(f, "null"),
        }
    }
}

