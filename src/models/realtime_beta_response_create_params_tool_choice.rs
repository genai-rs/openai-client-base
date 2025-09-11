use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeBetaResponseCreateParamsTool_choice - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeBetaResponseCreateParamsTool_choice {
    Toolchoiceoptions(models::ToolChoiceOptions),
    Toolchoicefunction(models::ToolChoiceFunction),
    Toolchoicemcp(models::ToolChoiceMCP),
}

impl Default for RealtimeBetaResponseCreateParamsTool_choice {
    fn default() -> Self {
        Self::Toolchoiceoptions(Default::default())
    }
}

