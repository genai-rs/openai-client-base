use crate::models;
use serde::{Deserialize, Serialize};

/// BetaCustomToolCallOutputOutput - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaCustomToolCallOutputOutput {
    StringOutput(String),
    OutputContentList(Vec<models::BetaFunctionAndCustomToolCallOutput>),
}

impl Default for BetaCustomToolCallOutputOutput {
    fn default() -> Self {
        Self::StringOutput(String::new())
    }
}

impl BetaCustomToolCallOutputOutput {
    pub fn new_text(text: String) -> Self {
        Self::StringOutput(text)
    }
    pub fn new_outputcontentlist(items: Vec<models::BetaFunctionAndCustomToolCallOutput>) -> Self {
        Self::OutputContentList(items)
    }
}

impl From<String> for BetaCustomToolCallOutputOutput {
    fn from(s: String) -> Self {
        Self::StringOutput(s)
    }
}

impl From<&str> for BetaCustomToolCallOutputOutput {
    fn from(s: &str) -> Self {
        Self::StringOutput(s.to_string())
    }
}
impl std::fmt::Display for BetaCustomToolCallOutputOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaCustomToolCallOutputOutput::StringOutput(value) => write!(f, "{}", value),
            BetaCustomToolCallOutputOutput::OutputContentList(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
