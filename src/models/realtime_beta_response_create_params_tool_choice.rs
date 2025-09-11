use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeBetaResponseCreateParamsTool_choice - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeBetaResponseCreateParamsTool_choice {
    ToolChoiceOptions(models::ToolChoiceOptions),
    ToolChoiceFunction(models::ToolChoiceFunction),
    ToolChoiceMCP(models::ToolChoiceMCP),
}

impl Default for RealtimeBetaResponseCreateParamsTool_choice {
    fn default() -> Self {
        Self::ToolChoiceOptions(Default::default())
    }
}

