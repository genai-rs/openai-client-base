use crate::models;
use serde::{Deserialize, Serialize};

/// DecisionInputJsonData - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DecisionInputJsonData {
    Arrayofobjects(Vec<serde_json::Value>),
}

impl std::fmt::Display for DecisionInputJsonData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecisionInputJsonData::Arrayofobjects(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}
