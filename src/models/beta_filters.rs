use crate::models;
use serde::{Deserialize, Serialize};

/// BetaFilters - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaFilters {
    Betacomparisonfilter(models::BetaComparisonFilter),
    Betacompoundfilter(models::BetaCompoundFilter),
}

impl std::fmt::Display for BetaFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaFilters::Betacomparisonfilter(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            BetaFilters::Betacompoundfilter(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}
