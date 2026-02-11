use crate::models;
use serde::{Deserialize, Serialize};

/// FunctionShellToolParamEnvironment - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FunctionShellToolParamEnvironment {
    Containerautoparam(models::ContainerAutoParam),
    Localenvironmentparam(models::LocalEnvironmentParam),
    Containerreferenceparam(models::ContainerReferenceParam),
    Null,
}

impl std::fmt::Display for FunctionShellToolParamEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionShellToolParamEnvironment::Containerautoparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            FunctionShellToolParamEnvironment::Localenvironmentparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            FunctionShellToolParamEnvironment::Containerreferenceparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            FunctionShellToolParamEnvironment::Null => write!(f, "null"),
        }
    }
}
