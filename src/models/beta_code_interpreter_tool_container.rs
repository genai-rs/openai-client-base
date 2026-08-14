use crate::models;
use serde::{Deserialize, Serialize};

/// BetaCodeInterpreterToolContainer - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaCodeInterpreterToolContainer {
    Text(String),
    Betaautocodeinterpretertoolparam(models::BetaAutoCodeInterpreterToolParam),
}

impl Default for BetaCodeInterpreterToolContainer {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl BetaCodeInterpreterToolContainer {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for BetaCodeInterpreterToolContainer {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for BetaCodeInterpreterToolContainer {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for BetaCodeInterpreterToolContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaCodeInterpreterToolContainer::Text(value) => write!(f, "{}", value),
            BetaCodeInterpreterToolContainer::Betaautocodeinterpretertoolparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
