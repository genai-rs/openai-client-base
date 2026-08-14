use crate::models;
use serde::{Deserialize, Serialize};

/// BetaFunctionShellCallItemParamEnvironment - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaFunctionShellCallItemParamEnvironment {
    Betalocalenvironmentparam(models::BetaLocalEnvironmentParam),
    Betacontainerreferenceparam(models::BetaContainerReferenceParam),
    Null,
}

impl std::fmt::Display for BetaFunctionShellCallItemParamEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaFunctionShellCallItemParamEnvironment::Betalocalenvironmentparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            BetaFunctionShellCallItemParamEnvironment::Betacontainerreferenceparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            BetaFunctionShellCallItemParamEnvironment::Null => write!(f, "null"),
        }
    }
}
