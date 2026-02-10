use crate::models;
use serde::{Deserialize, Serialize};

/// CreateTranscriptionResponseJsonUsage - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTranscriptionResponseJsonUsage {
    Transcripttextusagetokens(models::TranscriptTextUsageTokens),
    Transcripttextusageduration(models::TranscriptTextUsageDuration),
}

impl std::fmt::Display for CreateTranscriptionResponseJsonUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateTranscriptionResponseJsonUsage::Transcripttextusagetokens(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            CreateTranscriptionResponseJsonUsage::Transcripttextusageduration(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
