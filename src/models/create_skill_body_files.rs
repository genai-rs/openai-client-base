use crate::models;
use serde::{Deserialize, Serialize};

/// CreateSkillBodyFiles - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSkillBodyFiles {
    ArrayOfStrings(Vec<String>),
    Text(String),
}

impl Default for CreateSkillBodyFiles {
    fn default() -> Self {
        Self::ArrayOfStrings(Vec::new())
    }
}

impl std::fmt::Display for CreateSkillBodyFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateSkillBodyFiles::ArrayOfStrings(value) => match serde_json::to_string(value) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => Err(std::fmt::Error),
            },
            CreateSkillBodyFiles::Text(value) => write!(f, "{}", value),
        }
    }
}
