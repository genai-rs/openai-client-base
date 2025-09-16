use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeBetaResponseCreateParamsToolChoice - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeBetaResponseCreateParamsToolChoice {
    Toolchoiceoptions(models::ToolChoiceOptions),
    Toolchoicefunction(models::ToolChoiceFunction),
    Toolchoicemcp(models::ToolChoiceMcp),
}

impl std::fmt::Display for RealtimeBetaResponseCreateParamsToolChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RealtimeBetaResponseCreateParamsToolChoice::Toolchoiceoptions(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            RealtimeBetaResponseCreateParamsToolChoice::Toolchoicefunction(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            RealtimeBetaResponseCreateParamsToolChoice::Toolchoicemcp(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
