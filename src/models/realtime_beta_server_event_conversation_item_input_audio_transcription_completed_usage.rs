use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeBetaServerEventConversationItemInputAudioTranscriptionCompletedUsage - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeBetaServerEventConversationItemInputAudioTranscriptionCompletedUsage {
    Transcripttextusagetokens(models::TranscriptTextUsageTokens),
    Transcripttextusageduration(models::TranscriptTextUsageDuration),
}
