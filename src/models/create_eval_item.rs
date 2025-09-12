use crate::models;
use serde::{Deserialize, Serialize};

/// CreateEvalItem - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEvalItem {
    Evalitem(models::EvalItem),
}
