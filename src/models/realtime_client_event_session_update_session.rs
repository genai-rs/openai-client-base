use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeClientEventSessionUpdateSession - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeClientEventSessionUpdateSession {
    RealtimeSessionCreateRequestGA(models::RealtimeSessionCreateRequestGA),
    RealtimeTranscriptionSessionCreateRequestGA(models::RealtimeTranscriptionSessionCreateRequestGA),
}

impl Default for RealtimeClientEventSessionUpdateSession {
    fn default() -> Self {
        Self::RealtimeSessionCreateRequestGA(Default::default())
    }
}

