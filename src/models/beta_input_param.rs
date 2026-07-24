use crate::models;
use serde::{Deserialize, Serialize};

/// BetaInputParam - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaInputParam {
    TextInput(String),
    InputItemList(Vec<models::BetaInputItem>),
}

impl Default for BetaInputParam {
    fn default() -> Self {
        Self::TextInput(String::new())
    }
}

impl BetaInputParam {
    pub fn new_text(text: String) -> Self {
        Self::TextInput(text)
    }
    pub fn new_inputitemlist(items: Vec<models::BetaInputItem>) -> Self {
        Self::InputItemList(items)
    }
}

impl From<String> for BetaInputParam {
    fn from(s: String) -> Self {
        Self::TextInput(s)
    }
}

impl From<&str> for BetaInputParam {
    fn from(s: &str) -> Self {
        Self::TextInput(s.to_string())
    }
}
impl std::fmt::Display for BetaInputParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaInputParam::TextInput(value) => write!(f, "{}", value),
            BetaInputParam::InputItemList(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}
