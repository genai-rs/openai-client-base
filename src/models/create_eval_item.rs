use crate::models;
use serde::{Deserialize, Serialize};

/// CreateEvalItem - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEvalItem {
    Evalitem(models::EvalItem),
}

impl std::fmt::Display for CreateEvalItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateEvalItem::Evalitem(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}

