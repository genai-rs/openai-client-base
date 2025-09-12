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
