use crate::models;
use serde::{Deserialize, Serialize};

/// FunctionShellCallEnvironment - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FunctionShellCallEnvironment {
    Localenvironmentresource(models::LocalEnvironmentResource),
    Containerreferenceresource(models::ContainerReferenceResource),
    Null,
}

impl std::fmt::Display for FunctionShellCallEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionShellCallEnvironment::Localenvironmentresource(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            FunctionShellCallEnvironment::Containerreferenceresource(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            FunctionShellCallEnvironment::Null => write!(f, "null"),
        }
    }
}
