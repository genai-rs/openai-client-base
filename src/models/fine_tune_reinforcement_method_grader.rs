use crate::models;
use serde::{Deserialize, Serialize};

/// FineTuneReinforcementMethodGrader - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneReinforcementMethodGrader {
    Graderstringcheck(models::GraderStringCheck),
    Gradertextsimilarity(models::GraderTextSimilarity),
    Graderpython(models::GraderPython),
    Graderscoremodel(models::GraderScoreModel),
    Gradermulti(models::GraderMulti),
}

