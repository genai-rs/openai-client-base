use crate::models;
use serde::{Deserialize, Serialize};

/// VaultStatusFilterParam - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VaultStatusFilterParam {
    Vaultstatusparam(models::VaultStatusParam),
    Arrayofvaultstatusparams(Vec<models::VaultStatusParam>),
}

impl std::fmt::Display for VaultStatusFilterParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VaultStatusFilterParam::Vaultstatusparam(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            VaultStatusFilterParam::Arrayofvaultstatusparams(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
