use crate::models;
use serde::{Deserialize, Serialize};

/// LiveResponsesDelegationSettingsInputParamToolChoice - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LiveResponsesDelegationSettingsInputParamToolChoice {
    Livetoolchoiceenum(models::LiveToolChoiceEnum),
    Livefunctiontoolchoiceparam(models::LiveFunctionToolChoiceParam),
    Livemcptoolchoiceparam(models::LiveMcpToolChoiceParam),
}

impl std::fmt::Display for LiveResponsesDelegationSettingsInputParamToolChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiveResponsesDelegationSettingsInputParamToolChoice::Livetoolchoiceenum(value) => {
                write!(f, "{}", value)
            }
            LiveResponsesDelegationSettingsInputParamToolChoice::Livefunctiontoolchoiceparam(
                value,
            ) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            LiveResponsesDelegationSettingsInputParamToolChoice::Livemcptoolchoiceparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
