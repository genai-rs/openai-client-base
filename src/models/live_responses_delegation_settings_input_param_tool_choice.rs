use crate::models;
use serde::{Deserialize, Serialize};

/// LiveResponsesDelegationSettingsInputParamToolChoice - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LiveResponsesDelegationSettingsInputParamToolChoice {
    Livetoolchoiceenum(models::LiveToolChoiceEnum),
    Livefunctiontoolchoiceparam(models::LiveFunctionToolChoiceParam),
    Livemcptoolchoiceparam(models::LiveMcpToolChoiceParam),
    Livespecificfilesearchparam(models::LiveSpecificFileSearchParam),
    Livespecificwebsearchparam(models::LiveSpecificWebSearchParam),
    Livespecificwebsearchpreviewparam(models::LiveSpecificWebSearchPreviewParam),
    Livespecificimagegenparam(models::LiveSpecificImageGenParam),
    Livespecificcomputerparam(models::LiveSpecificComputerParam),
    Livespecificcodeinterpreterparam(models::LiveSpecificCodeInterpreterParam),
    Livespecificprogrammatictoolcallingparam(models::LiveSpecificProgrammaticToolCallingParam),
    Livespecificfunctionshellparam(models::LiveSpecificFunctionShellParam),
    Livespecificcustomtoolparam(models::LiveSpecificCustomToolParam),
    Livespecificapplypatchparam(models::LiveSpecificApplyPatchParam),
    Liveallowedtoolschoiceparam(models::LiveAllowedToolsChoiceParam),
}

impl std::fmt::Display for LiveResponsesDelegationSettingsInputParamToolChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiveResponsesDelegationSettingsInputParamToolChoice::Livetoolchoiceenum(value) => write!(f, "{}", value),
            LiveResponsesDelegationSettingsInputParamToolChoice::Livefunctiontoolchoiceparam(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            LiveResponsesDelegationSettingsInputParamToolChoice::Livemcptoolchoiceparam(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            LiveResponsesDelegationSettingsInputParamToolChoice::Livespecificfilesearchparam(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            LiveResponsesDelegationSettingsInputParamToolChoice::Livespecificwebsearchparam(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            LiveResponsesDelegationSettingsInputParamToolChoice::Livespecificwebsearchpreviewparam(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            LiveResponsesDelegationSettingsInputParamToolChoice::Livespecificimagegenparam(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            LiveResponsesDelegationSettingsInputParamToolChoice::Livespecificcomputerparam(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            LiveResponsesDelegationSettingsInputParamToolChoice::Livespecificcodeinterpreterparam(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            LiveResponsesDelegationSettingsInputParamToolChoice::Livespecificprogrammatictoolcallingparam(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            LiveResponsesDelegationSettingsInputParamToolChoice::Livespecificfunctionshellparam(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            LiveResponsesDelegationSettingsInputParamToolChoice::Livespecificcustomtoolparam(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            LiveResponsesDelegationSettingsInputParamToolChoice::Livespecificapplypatchparam(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            LiveResponsesDelegationSettingsInputParamToolChoice::Liveallowedtoolschoiceparam(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}
