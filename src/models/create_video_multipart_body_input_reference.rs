use crate::models;
use serde::{Deserialize, Serialize};

/// CreateVideoMultipartBodyInputReference - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateVideoMultipartBodyInputReference {
    Text(String),
    Imagerefparam2(models::ImageRefParam2),
}

impl Default for CreateVideoMultipartBodyInputReference {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl CreateVideoMultipartBodyInputReference {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for CreateVideoMultipartBodyInputReference {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for CreateVideoMultipartBodyInputReference {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for CreateVideoMultipartBodyInputReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateVideoMultipartBodyInputReference::Text(value) => write!(f, "{}", value),
            CreateVideoMultipartBodyInputReference::Imagerefparam2(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
