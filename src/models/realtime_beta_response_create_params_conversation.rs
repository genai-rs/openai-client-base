use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeBetaResponseCreateParamsConversation - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeBetaResponseCreateParamsConversation {
    Text(String),
    Text(TextEnum),
}

impl Default for RealtimeBetaResponseCreateParamsConversation {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl RealtimeBetaResponseCreateParamsConversation {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for RealtimeBetaResponseCreateParamsConversation {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for RealtimeBetaResponseCreateParamsConversation {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
