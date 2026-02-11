use crate::models;
use serde::{Deserialize, Serialize};

/// FunctionShellCallItemParamEnvironment - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FunctionShellCallItemParamEnvironment {
    Localenvironmentparam(models::LocalEnvironmentParam),
    Containerreferenceparam(models::ContainerReferenceParam),
    Null,
}

impl std::fmt::Display for FunctionShellCallItemParamEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionShellCallItemParamEnvironment::Localenvironmentparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            FunctionShellCallItemParamEnvironment::Containerreferenceparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            FunctionShellCallItemParamEnvironment::Null => write!(f, "null"),
        }
    }
}
