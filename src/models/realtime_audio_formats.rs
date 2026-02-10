use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeAudioFormats - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeAudioFormats {
    PcmAudioFormat(models::ChatCompletionNamedToolChoice),
    PcmuAudioFormat(models::ChatCompletionNamedToolChoice),
    PcmaAudioFormat(models::ChatCompletionNamedToolChoice),
}

impl std::fmt::Display for RealtimeAudioFormats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RealtimeAudioFormats::PcmAudioFormat(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            RealtimeAudioFormats::PcmuAudioFormat(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            RealtimeAudioFormats::PcmaAudioFormat(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}
