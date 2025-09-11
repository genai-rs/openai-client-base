use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionToolChoiceOption - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionToolChoiceOption {
    Auto(AutoEnum),
    Chatcompletionallowedtoolschoice(models::ChatCompletionAllowedToolsChoice),
    Chatcompletionnamedtoolchoice(models::ChatCompletionNamedToolChoice),
    Chatcompletionnamedtoolchoicecustom(models::ChatCompletionNamedToolChoiceCustom),
}

/// AutoEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AutoEnum {
    None,
    Auto,
    Required,
}

impl Default for AutoEnum {
    fn default() -> Self {
        Self::None
    }
}
