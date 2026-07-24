use crate::models;
use serde::{Deserialize, Serialize};

/// BetaMcpToolAllowedTools - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaMcpToolAllowedTools {
    ArrayOfStrings(Vec<String>),
    Betamcptoolfilter(models::BetaMcpToolFilter),
    Null,
}

impl Default for BetaMcpToolAllowedTools {
    fn default() -> Self {
        Self::ArrayOfStrings(Vec::new())
    }
}

impl std::fmt::Display for BetaMcpToolAllowedTools {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaMcpToolAllowedTools::ArrayOfStrings(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            BetaMcpToolAllowedTools::Betamcptoolfilter(value) => match serde_json::to_string(value)
            {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            BetaMcpToolAllowedTools::Null => write!(f, "null"),
        }
    }
}
