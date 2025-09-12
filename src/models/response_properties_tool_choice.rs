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
    Toolchoicemcp(models::ToolChoiceMCP),
    Toolchoicecustom(models::ToolChoiceCustom),
}

