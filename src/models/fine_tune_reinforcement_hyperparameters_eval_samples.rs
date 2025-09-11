use crate::models;
use serde::{Deserialize, Serialize};

/// FineTuneReinforcementHyperparametersEval_samples - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneReinforcementHyperparametersEval_samples {
    Text(TextEnum),
}

impl Default for FineTuneReinforcementHyperparametersEval_samples {
    fn default() -> Self {
        Self::Text(Default::default())
    }
}

