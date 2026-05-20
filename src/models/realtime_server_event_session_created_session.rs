use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeServerEventSessionCreatedSession - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeServerEventSessionCreatedSession {
    Realtimesessioncreateresponsega(models::RealtimeSessionCreateResponseGa),
    Realtimetranscriptionsessioncreateresponsega(
        models::RealtimeTranscriptionSessionCreateResponseGa,
    ),
}

impl std::fmt::Display for RealtimeServerEventSessionCreatedSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RealtimeServerEventSessionCreatedSession::Realtimesessioncreateresponsega(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            RealtimeServerEventSessionCreatedSession::Realtimetranscriptionsessioncreateresponsega(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}
