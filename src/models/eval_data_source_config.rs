use crate::models;
use serde::{Deserialize, Serialize};

/// EvalDataSourceConfig - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EvalDataSourceConfig {
    Evalcustomdatasourceconfig(models::EvalCustomDataSourceConfig),
    Evallogsdatasourceconfig(models::EvalLogsDataSourceConfig),
    Evalstoredcompletionsdatasourceconfig(models::EvalStoredCompletionsDataSourceConfig),
}

impl std::fmt::Display for EvalDataSourceConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalDataSourceConfig::Evalcustomdatasourceconfig(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            EvalDataSourceConfig::Evallogsdatasourceconfig(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            EvalDataSourceConfig::Evalstoredcompletionsdatasourceconfig(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
