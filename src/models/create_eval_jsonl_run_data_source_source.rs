use crate::models;
use serde::{Deserialize, Serialize};

/// CreateEvalJsonlRunDataSourceSource - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEvalJsonlRunDataSourceSource {
    Evaljsonlfilecontentsource(models::EvalJsonlFileContentSource),
    Evaljsonlfileidsource(models::EvalJsonlFileIdSource),
}

impl std::fmt::Display for CreateEvalJsonlRunDataSourceSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateEvalJsonlRunDataSourceSource::Evaljsonlfilecontentsource(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            CreateEvalJsonlRunDataSourceSource::Evaljsonlfileidsource(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
