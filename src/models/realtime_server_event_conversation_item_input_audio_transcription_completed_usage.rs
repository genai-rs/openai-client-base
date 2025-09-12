use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsage - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsage {
    Transcripttextusagetokens(models::TranscriptTextUsageTokens),
    Transcripttextusageduration(models::TranscriptTextUsageDuration),
}

