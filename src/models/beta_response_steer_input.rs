use crate::models;
use serde::{Deserialize, Serialize};

/// BetaResponseSteerInput - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaResponseSteerInput {
    TextInput(String),
    Betaresponsesteerinputitemlist(Vec<models::BetaResponseSteerInputItem>),
}

impl Default for BetaResponseSteerInput {
    fn default() -> Self {
        Self::TextInput(String::new())
    }
}

impl BetaResponseSteerInput {
    pub fn new_text(text: String) -> Self {
        Self::TextInput(text)
    }
    pub fn new_betaresponsesteerinputitemlist(
        items: Vec<models::BetaResponseSteerInputItem>,
    ) -> Self {
        Self::Betaresponsesteerinputitemlist(items)
    }
}

impl From<String> for BetaResponseSteerInput {
    fn from(s: String) -> Self {
        Self::TextInput(s)
    }
}

impl From<&str> for BetaResponseSteerInput {
    fn from(s: &str) -> Self {
        Self::TextInput(s.to_string())
    }
}
impl std::fmt::Display for BetaResponseSteerInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaResponseSteerInput::TextInput(value) => write!(f, "{}", value),
            BetaResponseSteerInput::Betaresponsesteerinputitemlist(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
