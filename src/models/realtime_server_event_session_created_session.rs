use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeServerEventSessionCreatedSession - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeServerEventSessionCreatedSession {
    Realtimesessioncreaterequestga(models::RealtimeSessionCreateRequestGA),
    Realtimetranscriptionsessioncreaterequestga(models::RealtimeTranscriptionSessionCreateRequestGA),
}

impl Default for RealtimeServerEventSessionCreatedSession {
    fn default() -> Self {
        Self::Realtimesessioncreaterequestga(Default::default())
    }
}

