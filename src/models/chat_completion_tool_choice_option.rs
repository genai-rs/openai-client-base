use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionToolChoiceOption - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionToolChoiceOption {
    ToolChoiceMode(ChatCompletionToolChoiceOptionToolChoiceModeEnum),
    Chatcompletionallowedtoolschoice(models::ChatCompletionAllowedToolsChoice),
    Chatcompletionnamedtoolchoice(models::ChatCompletionNamedToolChoice),
    Chatcompletionnamedtoolchoicecustom(models::ChatCompletionNamedToolChoiceCustom),
}

impl std::fmt::Display for ChatCompletionToolChoiceOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatCompletionToolChoiceOption::ToolChoiceMode(value) => write!(f, "{}", value),
            ChatCompletionToolChoiceOption::Chatcompletionallowedtoolschoice(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            ChatCompletionToolChoiceOption::Chatcompletionnamedtoolchoice(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            ChatCompletionToolChoiceOption::Chatcompletionnamedtoolchoicecustom(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}

/// ChatCompletionToolChoiceOptionToolChoiceModeEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatCompletionToolChoiceOptionToolChoiceModeEnum {
    None,
    Auto,
    Required,
}

impl Default for ChatCompletionToolChoiceOptionToolChoiceModeEnum {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for ChatCompletionToolChoiceOptionToolChoiceModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ChatCompletionToolChoiceOptionToolChoiceModeEnum::None => "none",
            ChatCompletionToolChoiceOptionToolChoiceModeEnum::Auto => "auto",
            ChatCompletionToolChoiceOptionToolChoiceModeEnum::Required => "required",
        };
        write!(f, "{}", value)
    }
}
