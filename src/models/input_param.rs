use crate::models;
use serde::{Deserialize, Serialize};

/// InputParam - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputParam {
    TextInput(String),
    InputItemList(Vec<models::InputItem>),
}

impl Default for InputParam {
    fn default() -> Self {
        Self::TextInput(String::new())
    }
}

impl InputParam {
    pub fn new_text(text: String) -> Self {
        Self::TextInput(text)
    }
    pub fn new_inputitemlist(items: Vec<models::InputItem>) -> Self {
        Self::InputItemList(items)
    }
}

impl From<String> for InputParam {
    fn from(s: String) -> Self {
        Self::TextInput(s)
    }
}

impl From<&str> for InputParam {
    fn from(s: &str) -> Self {
        Self::TextInput(s.to_string())
    }
}
impl std::fmt::Display for InputParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputParam::TextInput(value) => write!(f, "{}", value),
            InputParam::InputItemList(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}
