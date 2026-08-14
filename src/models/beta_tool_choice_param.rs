use crate::models;
use serde::{Deserialize, Serialize};

/// BetaToolChoiceParam - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaToolChoiceParam {
    Betatoolchoiceoptions(models::BetaToolChoiceOptions),
    Betatoolchoiceallowed(models::BetaToolChoiceAllowed),
    Betatoolchoicetypes(models::BetaToolChoiceTypes),
    Betatoolchoicefunction(models::BetaToolChoiceFunction),
    Betatoolchoicemcp(models::BetaToolChoiceMcp),
    Betatoolchoicecustom(models::BetaToolChoiceCustom),
    Betaspecificprogrammatictoolcallingparam(models::BetaSpecificProgrammaticToolCallingParam),
    Betaspecificapplypatchparam(models::BetaSpecificApplyPatchParam),
    Betaspecificfunctionshellparam(models::BetaSpecificFunctionShellParam),
}

impl std::fmt::Display for BetaToolChoiceParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaToolChoiceParam::Betatoolchoiceoptions(value) => match serde_json::to_string(value)
            {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            BetaToolChoiceParam::Betatoolchoiceallowed(value) => match serde_json::to_string(value)
            {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            BetaToolChoiceParam::Betatoolchoicetypes(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            BetaToolChoiceParam::Betatoolchoicefunction(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            BetaToolChoiceParam::Betatoolchoicemcp(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            BetaToolChoiceParam::Betatoolchoicecustom(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            BetaToolChoiceParam::Betaspecificprogrammatictoolcallingparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            BetaToolChoiceParam::Betaspecificapplypatchparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            BetaToolChoiceParam::Betaspecificfunctionshellparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
