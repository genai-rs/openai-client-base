use crate::models;
use serde::{Deserialize, Serialize};

/// LiveHostedShellContainerAutoParamNetworkPolicy - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LiveHostedShellContainerAutoParamNetworkPolicy {
    Livecontainernetworkpolicydisabledparam(models::LiveContainerNetworkPolicyDisabledParam),
    Livehostedshellnetworkpolicyallowlistparam(models::LiveHostedShellNetworkPolicyAllowlistParam),
    Null,
}

impl std::fmt::Display for LiveHostedShellContainerAutoParamNetworkPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiveHostedShellContainerAutoParamNetworkPolicy::Livecontainernetworkpolicydisabledparam(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            LiveHostedShellContainerAutoParamNetworkPolicy::Livehostedshellnetworkpolicyallowlistparam(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            LiveHostedShellContainerAutoParamNetworkPolicy::Null => write!(f, "null"),
        }
    }
}
