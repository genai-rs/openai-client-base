use serde::{Deserialize, Serialize};

/// ModelIdsLiveTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModelIdsLiveTextVariantEnum {
    #[serde(rename = "gpt-live-1")]
    GptLive1,
}

impl Default for ModelIdsLiveTextVariantEnum {
    fn default() -> Self {
        Self::GptLive1
    }
}

impl std::fmt::Display for ModelIdsLiveTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ModelIdsLiveTextVariantEnum::GptLive1 => "gpt-live-1",
        };
        write!(f, "{}", value)
    }
}
