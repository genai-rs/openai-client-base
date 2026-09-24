use crate::models;
use serde::{Deserialize, Serialize};

/// PermissionErrorResponseError - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PermissionErrorResponseError {
    Error(models::Error),
    Text(String),
}

impl std::fmt::Display for PermissionErrorResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PermissionErrorResponseError::Error(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            PermissionErrorResponseError::Text(value) => write!(f, "{}", value),
        }
    }
}
