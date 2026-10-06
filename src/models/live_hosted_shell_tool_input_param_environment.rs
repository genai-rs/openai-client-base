use crate::models;
use serde::{Deserialize, Serialize};

/// LiveHostedShellToolInputParamEnvironment - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LiveHostedShellToolInputParamEnvironment {
    Livehostedshellcontainerautoparam(models::LiveHostedShellContainerAutoParam),
    Livecontainerreferenceparam(models::LiveContainerReferenceParam),
    Livelocalenvironmentparam(models::LiveLocalEnvironmentParam),
    Null,
}

impl std::fmt::Display for LiveHostedShellToolInputParamEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiveHostedShellToolInputParamEnvironment::Livehostedshellcontainerautoparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            LiveHostedShellToolInputParamEnvironment::Livecontainerreferenceparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            LiveHostedShellToolInputParamEnvironment::Livelocalenvironmentparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            LiveHostedShellToolInputParamEnvironment::Null => write!(f, "null"),
        }
    }
}
