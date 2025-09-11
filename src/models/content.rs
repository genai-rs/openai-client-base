use crate::models;
use serde::{Deserialize, Serialize};

/// Content - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Content {
    InputContentTypes(models::InputContent),
    OutputContentTypes(models::OutputContent),
}

