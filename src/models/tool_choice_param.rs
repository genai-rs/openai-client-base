use crate::models;
use serde::{Deserialize, Serialize};

/// ToolChoiceParam - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolChoiceParam {
    Toolchoiceoptions(models::ToolChoiceOptions),
    Toolchoiceallowed(models::ToolChoiceAllowed),
    Toolchoicetypes(models::ToolChoiceTypes),
    Toolchoicefunction(models::ToolChoiceFunction),
    Toolchoicemcp(models::ToolChoiceMcp),
    Toolchoicecustom(models::ToolChoiceCustom),
    Specificapplypatchparam(models::SpecificApplyPatchParam),
    Specificfunctionshellparam(models::SpecificFunctionShellParam),
}

impl std::fmt::Display for ToolChoiceParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolChoiceParam::Toolchoiceoptions(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            ToolChoiceParam::Toolchoiceallowed(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            ToolChoiceParam::Toolchoicetypes(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            ToolChoiceParam::Toolchoicefunction(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            ToolChoiceParam::Toolchoicemcp(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            ToolChoiceParam::Toolchoicecustom(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            ToolChoiceParam::Specificapplypatchparam(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            ToolChoiceParam::Specificfunctionshellparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
