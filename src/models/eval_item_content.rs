use crate::models;
use serde::{Deserialize, Serialize};

/// EvalItemContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EvalItemContent {
    Evalitemcontentitem(models::EvalItemContentItem),
    Evalitemcontentarray(Vec<models::EvalItemContentItem>),
}

impl std::fmt::Display for EvalItemContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalItemContent::Evalitemcontentitem(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            EvalItemContent::Evalitemcontentarray(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}
