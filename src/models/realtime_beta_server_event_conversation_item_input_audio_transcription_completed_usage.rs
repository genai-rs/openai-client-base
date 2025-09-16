use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeBetaServerEventConversationItemInputAudioTranscriptionCompletedUsage - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeBetaServerEventConversationItemInputAudioTranscriptionCompletedUsage {
    Transcripttextusagetokens(models::TranscriptTextUsageTokens),
    Transcripttextusageduration(models::TranscriptTextUsageDuration),
}

impl std::fmt::Display
    for RealtimeBetaServerEventConversationItemInputAudioTranscriptionCompletedUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RealtimeBetaServerEventConversationItemInputAudioTranscriptionCompletedUsage::Transcripttextusagetokens(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            RealtimeBetaServerEventConversationItemInputAudioTranscriptionCompletedUsage::Transcripttextusageduration(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}
