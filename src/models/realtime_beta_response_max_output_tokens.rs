use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeBetaResponseMax_output_tokens - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeBetaResponseMax_output_tokens {
    Text(TextEnum),
}

impl Default for RealtimeBetaResponseMax_output_tokens {
    fn default() -> Self {
        Self::Text(Default::default())
    }
}

