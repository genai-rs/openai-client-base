use crate::models;
use serde::{Deserialize, Serialize};

/// RealtimeBetaResponseCreateParamsMax_output_tokens - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeBetaResponseCreateParamsMax_output_tokens {
    Text(TextEnum),
}

impl Default for RealtimeBetaResponseCreateParamsMax_output_tokens {
    fn default() -> Self {
        Self::Text(Default::default())
    }
}

