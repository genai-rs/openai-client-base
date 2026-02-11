use crate::models;
use serde::{Deserialize, Serialize};

/// CreateSkillVersionBodyFiles - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSkillVersionBodyFiles {
    ArrayOfStrings(Vec<String>),
    Text(String),
}

impl Default for CreateSkillVersionBodyFiles {
    fn default() -> Self {
        Self::ArrayOfStrings(Vec::new())
    }
}

impl std::fmt::Display for CreateSkillVersionBodyFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateSkillVersionBodyFiles::ArrayOfStrings(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            CreateSkillVersionBodyFiles::Text(value) => write!(f, "{}", value),
        }
    }
}
