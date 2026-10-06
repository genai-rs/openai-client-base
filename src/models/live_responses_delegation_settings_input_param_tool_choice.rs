use crate::models;
use serde::{Deserialize, Serialize};

/// LiveResponsesDelegationSettingsInputParamToolChoice - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LiveResponsesDelegationSettingsInputParamToolChoice {
    Livetoolchoiceenum(models::LiveToolChoiceEnum),
}

impl std::fmt::Display for LiveResponsesDelegationSettingsInputParamToolChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiveResponsesDelegationSettingsInputParamToolChoice::Livetoolchoiceenum(value) => {
                write!(f, "{}", value)
            }
        }
    }
}
