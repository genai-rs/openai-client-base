use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeServerEventSessionCreatedSession - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeServerEventSessionCreatedSession {
    Realtimesessioncreaterequestga(models::RealtimeSessionCreateRequestGa),
    Realtimetranscriptionsessioncreaterequestga(
        models::RealtimeTranscriptionSessionCreateRequestGa,
    ),
}

impl std::fmt::Display for RealtimeServerEventSessionCreatedSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RealtimeServerEventSessionCreatedSession::Realtimesessioncreaterequestga(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            RealtimeServerEventSessionCreatedSession::Realtimetranscriptionsessioncreaterequestga(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}
