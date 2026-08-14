use crate::models;
use serde::{Deserialize, Serialize};

/// BetaComparisonFilterValue - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaComparisonFilterValue {
    Text(String),
    Arrayofitems(Vec<serde_json::Value>),
}

impl Default for BetaComparisonFilterValue {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl BetaComparisonFilterValue {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_arrayofitems(items: Vec<serde_json::Value>) -> Self {
        Self::Arrayofitems(items)
    }
}

impl From<String> for BetaComparisonFilterValue {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for BetaComparisonFilterValue {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for BetaComparisonFilterValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaComparisonFilterValue::Text(value) => write!(f, "{}", value),
            BetaComparisonFilterValue::Arrayofitems(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}
