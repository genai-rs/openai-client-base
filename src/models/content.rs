use crate::models;
use serde::{Deserialize, Serialize};

/// Content - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Content {
    Inputcontenttypes(models::InputContent),
    Outputcontenttypes(models::OutputContent),
}

impl Default for Content {
    fn default() -> Self {
        Self::Inputcontenttypes(Default::default())
    }
}

