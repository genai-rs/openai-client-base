use crate::models;
use serde::{Deserialize, Serialize};

/// ConversationParam - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConversationParam {
    ConversationId(String),
    Conversationparam2(models::ConversationParam2),
}

impl Default for ConversationParam {
    fn default() -> Self {
        Self::ConversationId(String::new())
    }
}

impl ConversationParam {
    pub fn new_text(text: String) -> Self {
        Self::ConversationId(text)
    }
}

impl From<String> for ConversationParam {
    fn from(s: String) -> Self {
        Self::ConversationId(s)
    }
}

impl From<&str> for ConversationParam {
    fn from(s: &str) -> Self {
        Self::ConversationId(s.to_string())
    }
}
impl std::fmt::Display for ConversationParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversationParam::ConversationId(value) => write!(f, "{}", value),
            ConversationParam::Conversationparam2(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}
