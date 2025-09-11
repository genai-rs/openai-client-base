use crate::models;
use serde::{Deserialize, Serialize};

/// GraderMultiGraders - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GraderMultiGraders {
    GraderStringCheck(models::GraderStringCheck),
    GraderTextSimilarity(models::GraderTextSimilarity),
    GraderPython(models::GraderPython),
    GraderScoreModel(models::GraderScoreModel),
    GraderLabelModel(models::GraderLabelModel),
}

impl Default for GraderMultiGraders {
    fn default() -> Self {
        Self::GraderStringCheck(Default::default())
    }
}

