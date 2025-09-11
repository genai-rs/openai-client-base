use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeServerEventSessionCreatedSession - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeServerEventSessionCreatedSession {
    Realtimesessioncreaterequestga(models::RealtimeSessionCreateRequestGa),
    Realtimetranscriptionsessioncreaterequestga(models::RealtimeTranscriptionSessionCreateRequestGa),
}

