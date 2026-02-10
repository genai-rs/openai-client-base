use crate::models;
use serde::{Deserialize, Serialize};

/// CreateEvalCompletionsRunDataSourceSource - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEvalCompletionsRunDataSourceSource {
    Evaljsonlfilecontentsource(models::EvalJsonlFileContentSource),
    Evaljsonlfileidsource(models::EvalJsonlFileIdSource),
    Evalstoredcompletionssource(models::EvalStoredCompletionsSource),
}

impl std::fmt::Display for CreateEvalCompletionsRunDataSourceSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateEvalCompletionsRunDataSourceSource::Evaljsonlfilecontentsource(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            CreateEvalCompletionsRunDataSourceSource::Evaljsonlfileidsource(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            CreateEvalCompletionsRunDataSourceSource::Evalstoredcompletionssource(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
