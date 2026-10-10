use crate::models;
use serde::{Deserialize, Serialize};

/// DecisionsUsagePermissionErrorResponseError - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DecisionsUsagePermissionErrorResponseError {
    Decisionsusageerrordetails(models::DecisionsUsageErrorDetails),
    Text(String),
}

impl std::fmt::Display for DecisionsUsagePermissionErrorResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecisionsUsagePermissionErrorResponseError::Decisionsusageerrordetails(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            DecisionsUsagePermissionErrorResponseError::Text(value) => write!(f, "{}", value),
        }
    }
}
