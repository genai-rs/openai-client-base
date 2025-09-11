use crate::models;
use serde::{Deserialize, Serialize};

/// FineTuneReinforcementHyperparametersEval_interval - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneReinforcementHyperparametersEval_interval {
    Text(TextEnum),
}

impl Default for FineTuneReinforcementHyperparametersEval_interval {
    fn default() -> Self {
        Self::Text(Default::default())
    }
}

