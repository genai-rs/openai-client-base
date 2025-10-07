use crate::models;
use serde::{Deserialize, Serialize};

/// CustomToolCallOutputOutput - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CustomToolCallOutputOutput {
    StringOutput(String),
    OutputContentList(Vec<models::FunctionAndCustomToolCallOutput>),
}

impl Default for CustomToolCallOutputOutput {
    fn default() -> Self {
        Self::StringOutput(String::new())
    }
}

impl CustomToolCallOutputOutput {
    pub fn new_text(text: String) -> Self {
        Self::StringOutput(text)
    }
    pub fn new_outputcontentlist(items: Vec<models::FunctionAndCustomToolCallOutput>) -> Self {
        Self::OutputContentList(items)
    }
}

impl From<String> for CustomToolCallOutputOutput {
    fn from(s: String) -> Self {
        Self::StringOutput(s)
    }
}

impl From<&str> for CustomToolCallOutputOutput {
    fn from(s: &str) -> Self {
        Self::StringOutput(s.to_string())
    }
}
impl std::fmt::Display for CustomToolCallOutputOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomToolCallOutputOutput::StringOutput(value) => write!(f, "{}", value),
            CustomToolCallOutputOutput::OutputContentList(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
