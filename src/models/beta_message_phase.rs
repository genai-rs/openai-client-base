use crate::models;
use serde::{Deserialize, Serialize};

/// BetaMessagePhase - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaMessagePhase {
    Betamessagephase2(models::BetaMessagePhase2),
    Null,
}

impl std::fmt::Display for BetaMessagePhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaMessagePhase::Betamessagephase2(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            BetaMessagePhase::Null => write!(f, "null"),
        }
    }
}
