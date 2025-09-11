use crate::models;
use serde::{Deserialize, Serialize};

/// CodeInterpreterToolContainer - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodeInterpreterToolContainer {
    Text(String),
    CodeInterpreterToolAuto(models::CodeInterpreterToolAuto),
}

impl Default for CodeInterpreterToolContainer {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl CodeInterpreterToolContainer {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for CodeInterpreterToolContainer {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for CodeInterpreterToolContainer {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
