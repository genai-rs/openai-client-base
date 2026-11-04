use serde::{Deserialize, Serialize};

/// VoiceIdsOrCustomVoice - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VoiceIdsOrCustomVoice {
    Text(String),
}

impl std::fmt::Display for VoiceIdsOrCustomVoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VoiceIdsOrCustomVoice::Text(value) => write!(f, "{}", value),
        }
    }
}
