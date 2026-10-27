use crate::models;
use serde::{Deserialize, Serialize};

/// QuestionParamPredicateInstructions - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuestionParamPredicateInstructions {
    Text(String),
    Arrayofdecisionquestioninputparts(Vec<models::DecisionQuestionInputPart>),
}

impl Default for QuestionParamPredicateInstructions {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl QuestionParamPredicateInstructions {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
    pub fn new_arrayofdecisionquestioninputparts(
        items: Vec<models::DecisionQuestionInputPart>,
    ) -> Self {
        Self::Arrayofdecisionquestioninputparts(items)
    }
}

impl From<String> for QuestionParamPredicateInstructions {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for QuestionParamPredicateInstructions {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for QuestionParamPredicateInstructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuestionParamPredicateInstructions::Text(value) => write!(f, "{}", value),
            QuestionParamPredicateInstructions::Arrayofdecisionquestioninputparts(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
