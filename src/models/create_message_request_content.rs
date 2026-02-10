use crate::models;
use serde::{Deserialize, Serialize};

/// CreateMessageRequestContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateMessageRequestContent {
    TextContent(String),
    ArrayOfContentParts(Vec<serde_json::Value>),
}

impl std::fmt::Display for CreateMessageRequestContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateMessageRequestContent::TextContent(value) => write!(f, "{}", value),
            CreateMessageRequestContent::ArrayOfContentParts(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
