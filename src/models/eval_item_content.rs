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
