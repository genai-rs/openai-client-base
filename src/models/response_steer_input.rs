use crate::models;
use serde::{Deserialize, Serialize};

/// ResponseSteerInput - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseSteerInput {
    TextInput(String),
    Responsesteerinputitemlist(Vec<models::ResponseSteerInputItem>),
}

impl Default for ResponseSteerInput {
    fn default() -> Self {
        Self::TextInput(String::new())
    }
}

impl ResponseSteerInput {
    pub fn new_text(text: String) -> Self {
        Self::TextInput(text)
    }
    pub fn new_responsesteerinputitemlist(items: Vec<models::ResponseSteerInputItem>) -> Self {
        Self::Responsesteerinputitemlist(items)
    }
}

impl From<String> for ResponseSteerInput {
    fn from(s: String) -> Self {
        Self::TextInput(s)
    }
}

impl From<&str> for ResponseSteerInput {
    fn from(s: &str) -> Self {
        Self::TextInput(s.to_string())
    }
}
impl std::fmt::Display for ResponseSteerInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseSteerInput::TextInput(value) => write!(f, "{}", value),
            ResponseSteerInput::Responsesteerinputitemlist(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
