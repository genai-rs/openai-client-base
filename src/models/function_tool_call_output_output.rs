use crate::models;
use serde::{Deserialize, Serialize};

/// FunctionToolCallOutputOutput - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FunctionToolCallOutputOutput {
    StringOutput(String),
    OutputContentList(Vec<models::FunctionAndCustomToolCallOutput>),
}

impl Default for FunctionToolCallOutputOutput {
    fn default() -> Self {
        Self::StringOutput(String::new())
    }
}

impl FunctionToolCallOutputOutput {
    pub fn new_text(text: String) -> Self {
        Self::StringOutput(text)
    }
    pub fn new_outputcontentlist(items: Vec<models::FunctionAndCustomToolCallOutput>) -> Self {
        Self::OutputContentList(items)
    }
}

impl From<String> for FunctionToolCallOutputOutput {
    fn from(s: String) -> Self {
        Self::StringOutput(s)
    }
}

impl From<&str> for FunctionToolCallOutputOutput {
    fn from(s: &str) -> Self {
        Self::StringOutput(s.to_string())
    }
}
impl std::fmt::Display for FunctionToolCallOutputOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionToolCallOutputOutput::StringOutput(value) => write!(f, "{}", value),
            FunctionToolCallOutputOutput::OutputContentList(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}

