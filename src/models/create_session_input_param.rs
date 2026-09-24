use crate::models;
use serde::{Deserialize, Serialize};

/// CreateSessionInputParam - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSessionInputParam {
    Text(String),
    Arrayofinputmessageparams(Vec<models::InputMessageParam>),
}

impl Default for CreateSessionInputParam {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl CreateSessionInputParam {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_arrayofinputmessageparams(items: Vec<models::InputMessageParam>) -> Self {
        Self::Arrayofinputmessageparams(items)
    }
}

impl From<String> for CreateSessionInputParam {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for CreateSessionInputParam {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for CreateSessionInputParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateSessionInputParam::Text(value) => write!(f, "{}", value),
            CreateSessionInputParam::Arrayofinputmessageparams(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
