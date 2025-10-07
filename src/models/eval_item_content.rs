use crate::models;
use serde::{Deserialize, Serialize};

/// EvalItemContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EvalItemContent {
    TextInput(String),
    Inputtextcontent(models::InputTextContent),
    OutputText(models::ChatCompletionNamedToolChoice),
    InputImage(models::ChatCompletionNamedToolChoice),
    Inputaudio(models::InputAudio),
    AnArrayOfInputTextInputImageAndInputAudio(Vec<serde_json::Value>),
}

impl Default for EvalItemContent {
    fn default() -> Self {
        Self::TextInput(String::new())
    }
}

impl EvalItemContent {
    pub fn new_text(text: String) -> Self {
        Self::TextInput(text)
    }
    pub fn new_anarrayofinputtextinputimageandinputaudio(items: Vec<serde_json::Value>) -> Self {
        Self::AnArrayOfInputTextInputImageAndInputAudio(items)
    }
}

impl From<String> for EvalItemContent {
    fn from(s: String) -> Self {
        Self::TextInput(s)
    }
}

impl From<&str> for EvalItemContent {
    fn from(s: &str) -> Self {
        Self::TextInput(s.to_string())
    }
}
impl std::fmt::Display for EvalItemContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalItemContent::TextInput(value) => write!(f, "{}", value),
            EvalItemContent::Inputtextcontent(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            EvalItemContent::OutputText(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            EvalItemContent::InputImage(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            EvalItemContent::Inputaudio(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            EvalItemContent::AnArrayOfInputTextInputImageAndInputAudio(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}

