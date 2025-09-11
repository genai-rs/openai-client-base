use crate::models;
use serde::{Deserialize, Serialize};

/// EvalItemContent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EvalItemContent {
    Textinput(String),
    InputTextContent(models::InputTextContent),
    Output text(models::ChatCompletionNamedToolChoice),
    Input image(models::ChatCompletionNamedToolChoice),
    InputAudio(models::InputAudio),
    AnarrayofInputtext,Inputimage,andInputaudio(Vec<serde_json::Value>),
}

impl Default for EvalItemContent {
    fn default() -> Self {
        Self::Textinput(String::new())
    }
}

impl EvalItemContent {
    pub fn new_text(text: String) -> Self {
        Self::Textinput(text)
    }
    pub fn new_anarrayofinputtext,inputimage,andinputaudio(items: Vec<serde_json::Value>) -> Self {
        Self::AnarrayofInputtext,Inputimage,andInputaudio(items)
    }
}

impl From<String> for EvalItemContent {
    fn from(s: String) -> Self {
        Self::Textinput(s)
    }
}

impl From<&str> for EvalItemContent {
    fn from(s: &str) -> Self {
        Self::Textinput(s.to_string())
    }
}
