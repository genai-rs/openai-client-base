use serde::{Deserialize, Serialize};

/// AutoEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AutoEnum {
    None,
    Auto,
    Required,
}

impl Default for AutoEnum {
    fn default() -> Self {
        Self::None
    }
}
