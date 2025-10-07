use crate::models;
use serde::{Deserialize, Serialize};

/// ModifyThreadRequestToolResources - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModifyThreadRequestToolResources {
    Null,
}

impl Default for ModifyThreadRequestToolResources {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for ModifyThreadRequestToolResources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModifyThreadRequestToolResources::Null => write!(f, "null"),
        }
    }
}
