use crate::models;
use serde::{Deserialize, Serialize};

/// CreateEvalCompletionsRunDataSourceInputMessages - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEvalCompletionsRunDataSourceInputMessages {
    Templateinputmessages(models::ChatCompletionNamedToolChoice),
    Itemreferenceinputmessages(models::ChatCompletionNamedToolChoice),
}

impl std::fmt::Display for CreateEvalCompletionsRunDataSourceInputMessages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateEvalCompletionsRunDataSourceInputMessages::Templateinputmessages(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            CreateEvalCompletionsRunDataSourceInputMessages::Itemreferenceinputmessages(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
