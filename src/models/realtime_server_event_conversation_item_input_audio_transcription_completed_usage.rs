use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsage - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsage {
    Transcripttextusagetokens(models::TranscriptTextUsageTokens),
    Transcripttextusageduration(models::TranscriptTextUsageDuration),
}

impl std::fmt::Display
    for RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsage::Transcripttextusagetokens(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsage::Transcripttextusageduration(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}
