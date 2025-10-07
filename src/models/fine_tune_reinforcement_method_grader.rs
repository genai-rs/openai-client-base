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

impl std::fmt::Display for FineTuneReinforcementMethodGrader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FineTuneReinforcementMethodGrader::Graderstringcheck(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            FineTuneReinforcementMethodGrader::Gradertextsimilarity(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            FineTuneReinforcementMethodGrader::Graderpython(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            FineTuneReinforcementMethodGrader::Graderscoremodel(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            FineTuneReinforcementMethodGrader::Gradermulti(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
