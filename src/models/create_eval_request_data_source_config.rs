use crate::models;
use serde::{Deserialize, Serialize};

/// CreateEvalRequestDataSourceConfig - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEvalRequestDataSourceConfig {
    Createevalcustomdatasourceconfig(models::CreateEvalCustomDataSourceConfig),
    Createevallogsdatasourceconfig(models::CreateEvalLogsDataSourceConfig),
    Createevalstoredcompletionsdatasourceconfig(
        models::CreateEvalStoredCompletionsDataSourceConfig,
    ),
}

impl std::fmt::Display for CreateEvalRequestDataSourceConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateEvalRequestDataSourceConfig::Createevalcustomdatasourceconfig(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            CreateEvalRequestDataSourceConfig::Createevallogsdatasourceconfig(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            CreateEvalRequestDataSourceConfig::Createevalstoredcompletionsdatasourceconfig(
                value,
            ) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}
