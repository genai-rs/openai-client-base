use crate::models;
use serde::{Deserialize, Serialize};

/// DecisionInputImageDetail - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DecisionInputImageDetail {
    Imagedetailparam(models::ImageDetailParam),
    Null,
}

impl std::fmt::Display for DecisionInputImageDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecisionInputImageDetail::Imagedetailparam(value) => match serde_json::to_string(value)
            {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            DecisionInputImageDetail::Null => write!(f, "null"),
        }
    }
}
