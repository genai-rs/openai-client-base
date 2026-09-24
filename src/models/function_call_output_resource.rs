use crate::models;
use serde::{Deserialize, Serialize};

/// FunctionCallOutputResource - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FunctionCallOutputResource {
    Text(String),
    Arrayofinputcontentresources(Vec<models::InputContentResource>),
}

impl Default for FunctionCallOutputResource {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl FunctionCallOutputResource {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_arrayofinputcontentresources(items: Vec<models::InputContentResource>) -> Self {
        Self::Arrayofinputcontentresources(items)
    }
}

impl From<String> for FunctionCallOutputResource {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for FunctionCallOutputResource {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for FunctionCallOutputResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionCallOutputResource::Text(value) => write!(f, "{}", value),
            FunctionCallOutputResource::Arrayofinputcontentresources(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
