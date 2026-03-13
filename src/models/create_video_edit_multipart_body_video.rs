use crate::models;
use serde::{Deserialize, Serialize};

/// CreateVideoEditMultipartBodyVideo - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateVideoEditMultipartBodyVideo {
    Text(String),
    Videoreferenceinputparam(models::VideoReferenceInputParam),
}

impl Default for CreateVideoEditMultipartBodyVideo {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl CreateVideoEditMultipartBodyVideo {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for CreateVideoEditMultipartBodyVideo {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for CreateVideoEditMultipartBodyVideo {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for CreateVideoEditMultipartBodyVideo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateVideoEditMultipartBodyVideo::Text(value) => write!(f, "{}", value),
            CreateVideoEditMultipartBodyVideo::Videoreferenceinputparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
