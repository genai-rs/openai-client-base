use crate::models;
use serde::{Deserialize, Serialize};

/// ResponsePropertiesToolChoice - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResponsePropertiesToolChoice {
    #[serde(rename = "toolchoiceoptions")]
    ToolChoiceOptions(Box<models::ToolChoiceOptions>),
    #[serde(rename = "allowed_tools")]
    ToolChoiceAllowed(Box<models::ToolChoiceAllowed>),
    #[serde(rename = "toolchoicetypes")]
    ToolChoiceTypes(Box<models::ToolChoiceTypes>),
    #[serde(rename = "function")]
    ToolChoiceFunction(Box<models::ToolChoiceFunction>),
    #[serde(rename = "mcp")]
    ToolChoiceMcp(Box<models::ToolChoiceMcp>),
    #[serde(rename = "custom")]
    ToolChoiceCustom(Box<models::ToolChoiceCustom>),
}
