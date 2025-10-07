use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeClientEventSessionUpdateSession - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeClientEventSessionUpdateSession {
    Realtimesessioncreaterequestga(models::RealtimeSessionCreateRequestGa),
    Realtimetranscriptionsessioncreaterequestga(
        models::RealtimeTranscriptionSessionCreateRequestGa,
    ),
}

impl std::fmt::Display for RealtimeClientEventSessionUpdateSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RealtimeClientEventSessionUpdateSession::Realtimesessioncreaterequestga(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            RealtimeClientEventSessionUpdateSession::Realtimetranscriptionsessioncreaterequestga(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}
