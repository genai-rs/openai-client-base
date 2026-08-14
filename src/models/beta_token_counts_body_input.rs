use crate::models;
use serde::{Deserialize, Serialize};

/// BetaTokenCountsBodyInput - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaTokenCountsBodyInput {
    Text(String),
    Arrayofbetainputitems(Vec<models::BetaInputItem>),
    Null,
}

impl Default for BetaTokenCountsBodyInput {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl BetaTokenCountsBodyInput {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_arrayofbetainputitems(items: Vec<models::BetaInputItem>) -> Self {
        Self::Arrayofbetainputitems(items)
    }
}

impl From<String> for BetaTokenCountsBodyInput {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for BetaTokenCountsBodyInput {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for BetaTokenCountsBodyInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaTokenCountsBodyInput::Text(value) => write!(f, "{}", value),
            BetaTokenCountsBodyInput::Arrayofbetainputitems(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            BetaTokenCountsBodyInput::Null => write!(f, "null"),
        }
    }
}
