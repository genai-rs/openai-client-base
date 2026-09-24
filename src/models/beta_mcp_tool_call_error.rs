use crate::models;
use serde::{Deserialize, Serialize};

/// BetaMcpToolCallError - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaMcpToolCallError {
    Betamcptoolcallerror(Box<models::BetaMcpToolCallError>),
    Null,
}

impl std::fmt::Display for BetaMcpToolCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaMcpToolCallError::Betamcptoolcallerror(value) => match serde_json::to_string(value)
            {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            BetaMcpToolCallError::Null => write!(f, "null"),
        }
    }
}
