use crate::models;
use serde::{Deserialize, Serialize};

/// CreateEvalItem - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEvalItem {
    EvalItem(models::EvalItem),
}

impl Default for CreateEvalItem {
    fn default() -> Self {
        Self::EvalItem(Default::default())
    }
}

