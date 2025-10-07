use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeMcpToolCallError - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeMcpToolCallError {
    Realtimemcpprotocolerror(models::RealtimeMcpProtocolError),
    Realtimemcptoolexecutionerror(models::RealtimeMcpToolExecutionError),
    Realtimemcphttperror(models::RealtimeMcphttpError),
    Null,
}

impl std::fmt::Display for RealtimeMcpToolCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RealtimeMcpToolCallError::Realtimemcpprotocolerror(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            RealtimeMcpToolCallError::Realtimemcptoolexecutionerror(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            RealtimeMcpToolCallError::Realtimemcphttperror(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            RealtimeMcpToolCallError::Null => write!(f, "null"),
        }
    }
}
