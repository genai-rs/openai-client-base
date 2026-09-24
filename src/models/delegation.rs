use crate::models;
use serde::{Deserialize, Serialize};

/// Delegation - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Delegation {
    Liveclientdelegationparam(models::LiveClientDelegationParam),
    Liveresponsesdelegationparam(models::LiveResponsesDelegationParam),
    Null,
}

impl std::fmt::Display for Delegation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Delegation::Liveclientdelegationparam(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            Delegation::Liveresponsesdelegationparam(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            Delegation::Null => write!(f, "null"),
        }
    }
}
