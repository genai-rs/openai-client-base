use crate::models;
use serde::{Deserialize, Serialize};

/// CodeInterpreterToolContainer - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodeInterpreterToolContainer {
    Text(String),
}

impl Default for CodeInterpreterToolContainer {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl std::fmt::Display for CodeInterpreterToolContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeInterpreterToolContainer::Text(value) => write!(f, "{}", value),
        }
    }
}
