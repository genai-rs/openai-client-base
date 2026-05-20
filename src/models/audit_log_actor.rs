use crate::models;
use serde::{Deserialize, Serialize};

/// AuditLogActor - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuditLogActor {
    Auditlogactor(Box<models::AuditLogActor>),
    Null,
}

impl std::fmt::Display for AuditLogActor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditLogActor::Auditlogactor(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            AuditLogActor::Null => write!(f, "null"),
        }
    }
}
