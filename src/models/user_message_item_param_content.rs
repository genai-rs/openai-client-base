use crate::models;
use serde::{Deserialize, Serialize};

/// UserMessageItemParamContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserMessageItemParamContent {
    Arrayofitems(Vec<serde_json::Value>),
    Text(String),
}

impl std::fmt::Display for UserMessageItemParamContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserMessageItemParamContent::Arrayofitems(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            UserMessageItemParamContent::Text(value) => write!(f, "{}", value),
        }
    }
}
