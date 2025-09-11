use crate::models;
use serde::{Deserialize, Serialize};

/// CreateEvalRunRequestData_source - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEvalRunRequestData_source {
    CreateEvalJsonlRunDataSource(models::CreateEvalJsonlRunDataSource),
    CreateEvalCompletionsRunDataSource(models::CreateEvalCompletionsRunDataSource),
    CreateEvalResponsesRunDataSource(models::CreateEvalResponsesRunDataSource),
}

impl Default for CreateEvalRunRequestData_source {
    fn default() -> Self {
        Self::CreateEvalJsonlRunDataSource(Default::default())
    }
}

