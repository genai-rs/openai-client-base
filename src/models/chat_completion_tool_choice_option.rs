use crate::models;
use serde::{Deserialize, Serialize};

/// ChatCompletionToolChoiceOption - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionToolChoiceOption {
    Auto(ChatCompletionToolChoiceOptionAutoEnum),
    Chatcompletionallowedtoolschoice(models::ChatCompletionAllowedToolsChoice),
    Chatcompletionnamedtoolchoice(models::ChatCompletionNamedToolChoice),
    Chatcompletionnamedtoolchoicecustom(models::ChatCompletionNamedToolChoiceCustom),
}

impl std::fmt::Display for ChatCompletionToolChoiceOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatCompletionToolChoiceOption::Auto(value) => write!(f, "{}", value),
            ChatCompletionToolChoiceOption::Chatcompletionallowedtoolschoice(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            ChatCompletionToolChoiceOption::Chatcompletionnamedtoolchoice(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            ChatCompletionToolChoiceOption::Chatcompletionnamedtoolchoicecustom(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}

/// ChatCompletionToolChoiceOptionAutoEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatCompletionToolChoiceOptionAutoEnum {
    None,
    Auto,
    Required,
}

impl Default for ChatCompletionToolChoiceOptionAutoEnum {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for ChatCompletionToolChoiceOptionAutoEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ChatCompletionToolChoiceOptionAutoEnum::None => "none",
            ChatCompletionToolChoiceOptionAutoEnum::Auto => "auto",
            ChatCompletionToolChoiceOptionAutoEnum::Required => "required",
        };
        write!(f, "{}", value)
    }
}
