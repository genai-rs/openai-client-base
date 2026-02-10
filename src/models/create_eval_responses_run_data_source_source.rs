use crate::models;
use serde::{Deserialize, Serialize};

/// CreateEvalResponsesRunDataSourceSource - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEvalResponsesRunDataSourceSource {
    Evaljsonlfilecontentsource(models::EvalJsonlFileContentSource),
    Evaljsonlfileidsource(models::EvalJsonlFileIdSource),
    Evalresponsessource(models::EvalResponsesSource),
}

impl std::fmt::Display for CreateEvalResponsesRunDataSourceSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateEvalResponsesRunDataSourceSource::Evaljsonlfilecontentsource(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            CreateEvalResponsesRunDataSourceSource::Evaljsonlfileidsource(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            CreateEvalResponsesRunDataSourceSource::Evalresponsessource(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
