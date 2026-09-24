use crate::models;
use serde::{Deserialize, Serialize};

/// LiveSessionCreateParamsDelegation - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LiveSessionCreateParamsDelegation {
    Liveclientdelegationparam(models::LiveClientDelegationParam),
    Liveresponsesdelegationparam(models::LiveResponsesDelegationParam),
    Null,
}

impl std::fmt::Display for LiveSessionCreateParamsDelegation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiveSessionCreateParamsDelegation::Liveclientdelegationparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            LiveSessionCreateParamsDelegation::Liveresponsesdelegationparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            LiveSessionCreateParamsDelegation::Null => write!(f, "null"),
        }
    }
}
