use crate::models;
use serde::{Deserialize, Serialize};

/// BetaTextResponseFormatConfiguration - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaTextResponseFormatConfiguration {
    Betaresponseformattext(models::BetaResponseFormatText),
    Betatextresponseformatjsonschema(models::BetaTextResponseFormatJsonSchema),
    Betaresponseformatjsonobject(models::BetaResponseFormatJsonObject),
}

impl std::fmt::Display for BetaTextResponseFormatConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaTextResponseFormatConfiguration::Betaresponseformattext(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            BetaTextResponseFormatConfiguration::Betatextresponseformatjsonschema(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            BetaTextResponseFormatConfiguration::Betaresponseformatjsonobject(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
