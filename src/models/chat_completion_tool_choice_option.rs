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

impl Default for ChatCompletionToolChoiceOption {
    fn default() -> Self {
        Self::Auto(Default::default())
    }
}

