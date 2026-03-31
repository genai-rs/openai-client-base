use crate::models;
use serde::{Deserialize, Serialize};

/// MessagePhase - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessagePhase {
    Messagephase2(models::MessagePhase2),
    Null,
}

impl std::fmt::Display for MessagePhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessagePhase::Messagephase2(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            MessagePhase::Null => write!(f, "null"),
        }
    }
}
