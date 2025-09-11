use crate::models;
use serde::{Deserialize, Serialize};

/// Content - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Content {
    Inputcontent(models::InputContent),
    Outputcontent(models::OutputContent),
}
