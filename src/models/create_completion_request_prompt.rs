use crate::models;
use serde::{Deserialize, Serialize};

/// CreateCompletionRequestPrompt - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCompletionRequestPrompt {
    Text(String),
    ArrayOfStrings(Vec<String>),
    ArrayOfIntegers(Vec<i32>),
    ArrayOfIntegerArrays(Vec<Vec<i32>>),
}

impl Default for CreateCompletionRequestPrompt {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl CreateCompletionRequestPrompt {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_arrayofstrings(items: Vec<String>) -> Self {
        Self::ArrayOfStrings(items)
    }
    pub fn new_arrayofintegers(items: Vec<i32>) -> Self {
        Self::ArrayOfIntegers(items)
    }
    pub fn new_arrayofintegerarrays(items: Vec<Vec<i32>>) -> Self {
        Self::ArrayOfIntegerArrays(items)
    }
}

impl From<String> for CreateCompletionRequestPrompt {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for CreateCompletionRequestPrompt {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for CreateCompletionRequestPrompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateCompletionRequestPrompt::Text(value) => write!(f, "{}", value),
            CreateCompletionRequestPrompt::ArrayOfStrings(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            CreateCompletionRequestPrompt::ArrayOfIntegers(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            CreateCompletionRequestPrompt::ArrayOfIntegerArrays(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
