use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeServerEventSessionCreatedSession - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeServerEventSessionCreatedSession {
    RealtimeSessionCreateRequestGA(models::RealtimeSessionCreateRequestGA),
    RealtimeTranscriptionSessionCreateRequestGA(models::RealtimeTranscriptionSessionCreateRequestGA),
}

impl Default for RealtimeServerEventSessionCreatedSession {
    fn default() -> Self {
        Self::RealtimeSessionCreateRequestGA(Default::default())
    }
}

