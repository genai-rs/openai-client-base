use crate::models;
use serde::{Deserialize, Serialize};

/// EvalRunDataSource - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EvalRunDataSource {
    Createevaljsonlrundatasource(models::CreateEvalJsonlRunDataSource),
    Createevalcompletionsrundatasource(models::CreateEvalCompletionsRunDataSource),
    Createevalresponsesrundatasource(models::CreateEvalResponsesRunDataSource),
}

impl std::fmt::Display for EvalRunDataSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalRunDataSource::Createevaljsonlrundatasource(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            EvalRunDataSource::Createevalcompletionsrundatasource(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            EvalRunDataSource::Createevalresponsesrundatasource(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
