use serde::{Deserialize, Serialize};

/// ResponsesonlymodelEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponsesonlymodelEnum {
    #[serde(rename = "o1-pro")]
    O1Pro,
    #[serde(rename = "o1-pro-2025-03-19")]
    O1Pro20250319,
    #[serde(rename = "o3-pro")]
    O3Pro,
    #[serde(rename = "o3-pro-2025-06-10")]
    O3Pro20250610,
    #[serde(rename = "o3-deep-research")]
    O3DeepResearch,
    #[serde(rename = "o3-deep-research-2025-06-26")]
    O3DeepResearch20250626,
    #[serde(rename = "o4-mini-deep-research")]
    O4MiniDeepResearch,
    #[serde(rename = "o4-mini-deep-research-2025-06-26")]
    O4MiniDeepResearch20250626,
    #[serde(rename = "computer-use-preview")]
    ComputerUsePreview,
    #[serde(rename = "computer-use-preview-2025-03-11")]
    ComputerUsePreview20250311,
}

impl Default for ResponsesonlymodelEnum {
    fn default() -> Self {
        Self::O1Pro
    }
}
