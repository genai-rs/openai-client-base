use crate::models;
use serde::{Deserialize, Serialize};

/// BetaFunctionCallOutputItemParamOutput - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaFunctionCallOutputItemParamOutput {
    Text(String),
    Arrayofitems(Vec<serde_json::Value>),
}

impl Default for BetaFunctionCallOutputItemParamOutput {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl BetaFunctionCallOutputItemParamOutput {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_arrayofitems(items: Vec<serde_json::Value>) -> Self {
        Self::Arrayofitems(items)
    }
}

impl From<String> for BetaFunctionCallOutputItemParamOutput {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for BetaFunctionCallOutputItemParamOutput {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for BetaFunctionCallOutputItemParamOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaFunctionCallOutputItemParamOutput::Text(value) => write!(f, "{}", value),
            BetaFunctionCallOutputItemParamOutput::Arrayofitems(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
