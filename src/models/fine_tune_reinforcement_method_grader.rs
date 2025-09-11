use crate::models;
use serde::{Deserialize, Serialize};

/// FineTuneReinforcementMethodGrader - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneReinforcementMethodGrader {
    GraderStringCheck(models::GraderStringCheck),
    GraderTextSimilarity(models::GraderTextSimilarity),
    GraderPython(models::GraderPython),
    GraderScoreModel(models::GraderScoreModel),
    GraderMulti(models::GraderMulti),
}

impl Default for FineTuneReinforcementMethodGrader {
    fn default() -> Self {
        Self::GraderStringCheck(Default::default())
    }
}

