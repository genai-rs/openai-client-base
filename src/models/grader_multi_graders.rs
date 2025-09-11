use crate::models;
use serde::{Deserialize, Serialize};

/// GraderMultiGraders - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GraderMultiGraders {
    Graderstringcheck(models::GraderStringCheck),
    Gradertextsimilarity(models::GraderTextSimilarity),
    Graderpython(models::GraderPython),
    Graderscoremodel(models::GraderScoreModel),
    Graderlabelmodel(models::GraderLabelModel),
}

impl Default for GraderMultiGraders {
    fn default() -> Self {
        Self::Graderstringcheck(Default::default())
    }
}

