use crate::models;
use serde::{Deserialize, Serialize};

/// ChoiceValueParam - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChoiceValueParam {
    Text(String),
}

impl Default for ChoiceValueParam {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl std::fmt::Display for ChoiceValueParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChoiceValueParam::Text(value) => write!(f, "{}", value),
        }
    }
}
