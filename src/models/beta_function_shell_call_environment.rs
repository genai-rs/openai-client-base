use crate::models;
use serde::{Deserialize, Serialize};

/// BetaFunctionShellCallEnvironment - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaFunctionShellCallEnvironment {
    Betalocalenvironmentresource(models::BetaLocalEnvironmentResource),
    Betacontainerreferenceresource(models::BetaContainerReferenceResource),
    Null,
}

impl std::fmt::Display for BetaFunctionShellCallEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaFunctionShellCallEnvironment::Betalocalenvironmentresource(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            BetaFunctionShellCallEnvironment::Betacontainerreferenceresource(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            BetaFunctionShellCallEnvironment::Null => write!(f, "null"),
        }
    }
}
