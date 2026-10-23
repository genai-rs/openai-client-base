use crate::models;
use serde::{Deserialize, Serialize};

/// QuestionParamScoreInstructions - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuestionParamScoreInstructions {
    Text(String),
    Arrayofdecisionquestioninputparts(Vec<models::DecisionQuestionInputPart>),
}

impl Default for QuestionParamScoreInstructions {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl QuestionParamScoreInstructions {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_arrayofdecisionquestioninputparts(
        items: Vec<models::DecisionQuestionInputPart>,
    ) -> Self {
        Self::Arrayofdecisionquestioninputparts(items)
    }
}

impl From<String> for QuestionParamScoreInstructions {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for QuestionParamScoreInstructions {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for QuestionParamScoreInstructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuestionParamScoreInstructions::Text(value) => write!(f, "{}", value),
            QuestionParamScoreInstructions::Arrayofdecisionquestioninputparts(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
