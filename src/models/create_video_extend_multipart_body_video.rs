use crate::models;
use serde::{Deserialize, Serialize};

/// CreateVideoExtendMultipartBodyVideo - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateVideoExtendMultipartBodyVideo {
    Videoreferenceinputparam(models::VideoReferenceInputParam),
    Text(String),
}

impl std::fmt::Display for CreateVideoExtendMultipartBodyVideo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateVideoExtendMultipartBodyVideo::Videoreferenceinputparam(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            CreateVideoExtendMultipartBodyVideo::Text(value) => write!(f, "{}", value),
        }
    }
}
