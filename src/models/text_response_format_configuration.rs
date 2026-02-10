use crate::models;
use serde::{Deserialize, Serialize};

/// TextResponseFormatConfiguration - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextResponseFormatConfiguration {
    Responseformattext(models::ResponseFormatText),
    Textresponseformatjsonschema(models::TextResponseFormatJsonSchema),
    Responseformatjsonobject(models::ResponseFormatJsonObject),
}

impl std::fmt::Display for TextResponseFormatConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextResponseFormatConfiguration::Responseformattext(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            TextResponseFormatConfiguration::Textresponseformatjsonschema(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            TextResponseFormatConfiguration::Responseformatjsonobject(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
