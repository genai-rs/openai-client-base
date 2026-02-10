use crate::models;
use serde::{Deserialize, Serialize};

/// CreateEvalResponsesRunDataSourceInputMessages - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEvalResponsesRunDataSourceInputMessages {
    Inputmessagestemplate(models::ChatCompletionNamedToolChoice),
    Inputmessagesitemreference(models::ChatCompletionNamedToolChoice),
}

impl std::fmt::Display for CreateEvalResponsesRunDataSourceInputMessages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateEvalResponsesRunDataSourceInputMessages::Inputmessagestemplate(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            CreateEvalResponsesRunDataSourceInputMessages::Inputmessagesitemreference(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
