use crate::models;
use serde::{Deserialize, Serialize};

/// EvalItemContentItem - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EvalItemContentItem {
    Text(String),
    Inputtextcontent(models::InputTextContent),
    Evalitemcontentoutputtext(models::EvalItemContentOutputText),
    Evaliteminputimage(models::EvalItemInputImage),
    Inputaudio(models::InputAudio),
}

impl Default for EvalItemContentItem {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl EvalItemContentItem {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for EvalItemContentItem {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for EvalItemContentItem {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for EvalItemContentItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalItemContentItem::Text(value) => write!(f, "{}", value),
            EvalItemContentItem::Inputtextcontent(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            EvalItemContentItem::Evalitemcontentoutputtext(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            EvalItemContentItem::Evaliteminputimage(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            EvalItemContentItem::Inputaudio(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}
