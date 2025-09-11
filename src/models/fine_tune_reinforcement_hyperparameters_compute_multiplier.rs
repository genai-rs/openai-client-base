use crate::models;
use serde::{Deserialize, Serialize};

/// FineTuneReinforcementHyperparametersCompute_multiplier - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneReinforcementHyperparametersCompute_multiplier {
    Text(TextEnum),
}

impl Default for FineTuneReinforcementHyperparametersCompute_multiplier {
    fn default() -> Self {
        Self::Text(Default::default())
    }
}

