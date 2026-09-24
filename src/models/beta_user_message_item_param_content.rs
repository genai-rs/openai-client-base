use crate::models;
use serde::{Deserialize, Serialize};

/// BetaUserMessageItemParamContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaUserMessageItemParamContent {
    Arrayofitems(Vec<serde_json::Value>),
    Text(String),
}

impl std::fmt::Display for BetaUserMessageItemParamContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaUserMessageItemParamContent::Arrayofitems(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            BetaUserMessageItemParamContent::Text(value) => write!(f, "{}", value),
        }
    }
}
