use crate::models;
use serde::{Deserialize, Serialize};

/// CreateModerationRequestInput - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateModerationRequestInput {
    Text(String),
    ArrayOfStrings(Vec<String>),
    Arrayofitems(Vec<serde_json::Value>),
}

impl Default for CreateModerationRequestInput {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl CreateModerationRequestInput {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_arrayofstrings(items: Vec<String>) -> Self {
        Self::ArrayOfStrings(items)
    }
    pub fn new_arrayofitems(items: Vec<serde_json::Value>) -> Self {
        Self::Arrayofitems(items)
    }
}

impl From<String> for CreateModerationRequestInput {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for CreateModerationRequestInput {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for CreateModerationRequestInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateModerationRequestInput::Text(value) => write!(f, "{}", value),
            CreateModerationRequestInput::ArrayOfStrings(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            CreateModerationRequestInput::Arrayofitems(value) => match serde_json::to_string(value)
            {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}
