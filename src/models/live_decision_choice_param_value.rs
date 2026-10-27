use serde::{Deserialize, Serialize};

/// LiveDecisionChoiceParamValue - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LiveDecisionChoiceParamValue {
    Text(String),
}

impl Default for LiveDecisionChoiceParamValue {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl std::fmt::Display for LiveDecisionChoiceParamValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiveDecisionChoiceParamValue::Text(value) => write!(f, "{}", value),
        }
    }
}
