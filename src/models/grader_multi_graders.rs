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

impl std::fmt::Display for GraderMultiGraders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraderMultiGraders::Graderstringcheck(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            GraderMultiGraders::Gradertextsimilarity(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            GraderMultiGraders::Graderpython(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            GraderMultiGraders::Graderscoremodel(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            GraderMultiGraders::Graderlabelmodel(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}

