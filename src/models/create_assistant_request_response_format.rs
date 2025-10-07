use crate::models;
use serde::{Deserialize, Serialize};

/// CreateAssistantRequestResponseFormat - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAssistantRequestResponseFormat {
    Assistantsapiresponseformatoption(models::AssistantsApiResponseFormatOption),
    Null,
}

impl std::fmt::Display for CreateAssistantRequestResponseFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateAssistantRequestResponseFormat::Assistantsapiresponseformatoption(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            CreateAssistantRequestResponseFormat::Null => write!(f, "null"),
        }
    }
}
