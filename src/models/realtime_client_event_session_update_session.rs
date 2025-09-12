use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeClientEventSessionUpdateSession - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeClientEventSessionUpdateSession {
    Realtimesessioncreaterequestga(models::RealtimeSessionCreateRequestGa),
    Realtimetranscriptionsessioncreaterequestga(models::RealtimeTranscriptionSessionCreateRequestGa),
}

