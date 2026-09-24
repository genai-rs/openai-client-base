use crate::models;
use serde::{Deserialize, Serialize};

/// LiveSessionUpdateParamsDelegation - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LiveSessionUpdateParamsDelegation {
    Liveclientdelegationparam(models::LiveClientDelegationParam),
    Liveresponsesdelegationupdateparam(models::LiveResponsesDelegationUpdateParam),
    Null,
}

impl std::fmt::Display for LiveSessionUpdateParamsDelegation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiveSessionUpdateParamsDelegation::Liveclientdelegationparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            LiveSessionUpdateParamsDelegation::Liveresponsesdelegationupdateparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            LiveSessionUpdateParamsDelegation::Null => write!(f, "null"),
        }
    }
}
