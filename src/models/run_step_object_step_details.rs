use crate::models;
use serde::{Deserialize, Serialize};

/// RunStepObjectStepDetails - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RunStepObjectStepDetails {
    Runstepdetailsmessagecreationobject(models::RunStepDetailsMessageCreationObject),
    Runstepdetailstoolcallsobject(models::RunStepDetailsToolCallsObject),
}

impl std::fmt::Display for RunStepObjectStepDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunStepObjectStepDetails::Runstepdetailsmessagecreationobject(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            RunStepObjectStepDetails::Runstepdetailstoolcallsobject(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
