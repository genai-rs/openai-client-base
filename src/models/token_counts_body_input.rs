use crate::models;
use serde::{Deserialize, Serialize};

/// TokenCountsBodyInput - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TokenCountsBodyInput {
    Text(String),
    Arrayofitems(Vec<models::InputItem>),
    Null,
}

impl Default for TokenCountsBodyInput {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl TokenCountsBodyInput {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_arrayofitems(items: Vec<models::InputItem>) -> Self {
        Self::Arrayofitems(items)
    }
}

impl From<String> for TokenCountsBodyInput {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for TokenCountsBodyInput {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for TokenCountsBodyInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenCountsBodyInput::Text(value) => write!(f, "{}", value),
            TokenCountsBodyInput::Arrayofitems(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            TokenCountsBodyInput::Null => write!(f, "null"),
        }
    }
}
