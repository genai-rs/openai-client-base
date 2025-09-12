use crate::models;
use serde::{Deserialize, Serialize};

/// ResponsePropertiesToolChoice - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponsePropertiesToolChoice {
    // String enum options: none | auto | required
    Options(models::ToolChoiceOptions),
    // Constrained allowed tools
    Toolchoiceallowed(models::ToolChoiceAllowed),
    // Hosted tool type selector
    Toolchoicetypes(models::ToolChoiceTypes),
    // Force function tool
    Toolchoicefunction(models::ToolChoiceFunction),
    // Force MCP tool
    Toolchoicemcp(models::ToolChoiceMcp),
    // Force custom tool
    Toolchoicecustom(models::ToolChoiceCustom),
}
