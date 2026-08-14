use crate::models;
use serde::{Deserialize, Serialize};

/// BetaContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaContent {
    Betainputcontent(models::BetaInputContent),
    Betaoutputcontent(models::BetaOutputContent),
}

impl std::fmt::Display for BetaContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaContent::Betainputcontent(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            BetaContent::Betaoutputcontent(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}
