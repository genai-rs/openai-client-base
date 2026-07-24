use crate::models;
use serde::{Deserialize, Serialize};

/// BetaFunctionShellToolParamEnvironment - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaFunctionShellToolParamEnvironment {
    Betacontainerautoparam(models::BetaContainerAutoParam),
    Betalocalenvironmentparam(models::BetaLocalEnvironmentParam),
    Betacontainerreferenceparam(models::BetaContainerReferenceParam),
    Null,
}

impl std::fmt::Display for BetaFunctionShellToolParamEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaFunctionShellToolParamEnvironment::Betacontainerautoparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            BetaFunctionShellToolParamEnvironment::Betalocalenvironmentparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            BetaFunctionShellToolParamEnvironment::Betacontainerreferenceparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            BetaFunctionShellToolParamEnvironment::Null => write!(f, "null"),
        }
    }
}
