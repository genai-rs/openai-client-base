use crate::models;
use serde::{Deserialize, Serialize};

/// CreateAssistantRequestTemperature - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAssistantRequestTemperature {
    Null,
}

impl Default for CreateAssistantRequestTemperature {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Display for CreateAssistantRequestTemperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateAssistantRequestTemperature::Null => write!(f, "null"),
        }
    }
}
