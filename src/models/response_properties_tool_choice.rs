use crate::models;
use serde::{Deserialize, Serialize};

/// ResponsePropertiesToolChoice - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponsePropertiesToolChoice {
    Options(models::ToolChoiceOptions),
    Toolchoiceallowed(models::ToolChoiceAllowed),
    Toolchoicetypes(models::ToolChoiceTypes),
    Toolchoicefunction(models::ToolChoiceFunction),
    Toolchoicemcp(models::ToolChoiceMcp),
    Toolchoicecustom(models::ToolChoiceCustom),
}

impl std::fmt::Display for ResponsePropertiesToolChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponsePropertiesToolChoice::Options(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            ResponsePropertiesToolChoice::Toolchoiceallowed(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            ResponsePropertiesToolChoice::Toolchoicetypes(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            ResponsePropertiesToolChoice::Toolchoicefunction(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            ResponsePropertiesToolChoice::Toolchoicemcp(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            ResponsePropertiesToolChoice::Toolchoicecustom(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}

