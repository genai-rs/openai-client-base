use crate::models;
use serde::{Deserialize, Serialize};

/// FunctionCallOutputItemParamOutput - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FunctionCallOutputItemParamOutput {
    Text(String),
    Array(Vec<serde_json::Value>),
}

impl Default for FunctionCallOutputItemParamOutput {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl FunctionCallOutputItemParamOutput {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_array(items: Vec<serde_json::Value>) -> Self {
        Self::Array(items)
    }
}

impl From<String> for FunctionCallOutputItemParamOutput {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for FunctionCallOutputItemParamOutput {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for FunctionCallOutputItemParamOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionCallOutputItemParamOutput::Text(value) => write!(f, "{}", value),
            FunctionCallOutputItemParamOutput::Array(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}

