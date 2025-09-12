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

