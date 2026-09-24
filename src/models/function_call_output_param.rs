use crate::models;
use serde::{Deserialize, Serialize};

/// FunctionCallOutputParam - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FunctionCallOutputParam {
    Text(String),
    Arrayofinputcontentparams(Vec<models::InputContentParam>),
}

impl Default for FunctionCallOutputParam {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl FunctionCallOutputParam {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_arrayofinputcontentparams(items: Vec<models::InputContentParam>) -> Self {
        Self::Arrayofinputcontentparams(items)
    }
}

impl From<String> for FunctionCallOutputParam {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for FunctionCallOutputParam {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for FunctionCallOutputParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionCallOutputParam::Text(value) => write!(f, "{}", value),
            FunctionCallOutputParam::Arrayofinputcontentparams(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
