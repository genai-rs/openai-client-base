use crate::models;
use serde::{Deserialize, Serialize};

/// AssistantStreamEvent - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssistantStreamEvent {
    Threadstreamevent(models::ThreadStreamEvent),
    Runstreamevent(models::RunStreamEvent),
    Runstepstreamevent(models::RunStepStreamEvent),
    Messagestreamevent(models::MessageStreamEvent),
    Errorevent(models::ErrorEvent),
    Doneevent(models::DoneEvent),
}

impl std::fmt::Display for AssistantStreamEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssistantStreamEvent::Threadstreamevent(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            AssistantStreamEvent::Runstreamevent(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            AssistantStreamEvent::Runstepstreamevent(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            AssistantStreamEvent::Messagestreamevent(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            AssistantStreamEvent::Errorevent(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            AssistantStreamEvent::Doneevent(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}
