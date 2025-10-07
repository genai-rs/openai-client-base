use crate::models;
use serde::{Deserialize, Serialize};

/// CreateEvalRunRequestDataSource - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEvalRunRequestDataSource {
    Createevaljsonlrundatasource(models::CreateEvalJsonlRunDataSource),
    Createevalcompletionsrundatasource(models::CreateEvalCompletionsRunDataSource),
    Createevalresponsesrundatasource(models::CreateEvalResponsesRunDataSource),
}

impl std::fmt::Display for CreateEvalRunRequestDataSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateEvalRunRequestDataSource::Createevaljsonlrundatasource(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            CreateEvalRunRequestDataSource::Createevalcompletionsrundatasource(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            CreateEvalRunRequestDataSource::Createevalresponsesrundatasource(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}

