use crate::models;
use serde::{Deserialize, Serialize};

/// BetaConversationParam - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaConversationParam {
    ConversationId(String),
    Betaconversationparam2(models::BetaConversationParam2),
}

impl Default for BetaConversationParam {
    fn default() -> Self {
        Self::ConversationId(String::new())
    }
}

impl BetaConversationParam {
    pub fn new_text(text: String) -> Self {
        Self::ConversationId(text)
    }
}

impl From<String> for BetaConversationParam {
    fn from(s: String) -> Self {
        Self::ConversationId(s)
    }
}

impl From<&str> for BetaConversationParam {
    fn from(s: &str) -> Self {
        Self::ConversationId(s.to_string())
    }
}
impl std::fmt::Display for BetaConversationParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetaConversationParam::ConversationId(value) => write!(f, "{}", value),
            BetaConversationParam::Betaconversationparam2(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
