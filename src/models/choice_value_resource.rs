use crate::models;
use serde::{Deserialize, Serialize};

/// ChoiceValueResource - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChoiceValueResource {
    Text(String),
}

impl Default for ChoiceValueResource {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl std::fmt::Display for ChoiceValueResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChoiceValueResource::Text(value) => write!(f, "{}", value),
        }
    }
}
