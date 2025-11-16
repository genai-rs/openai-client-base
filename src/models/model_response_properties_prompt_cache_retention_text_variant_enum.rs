use serde::{Deserialize, Serialize};

/// ModelResponsePropertiesPromptCacheRetentionTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModelResponsePropertiesPromptCacheRetentionTextVariantEnum {
    #[serde(rename = "in-memory")]
    InMemory,
    #[serde(rename = "24h")]
    Variant24h,
}

impl Default for ModelResponsePropertiesPromptCacheRetentionTextVariantEnum {
    fn default() -> Self {
        Self::InMemory
    }
}

impl std::fmt::Display for ModelResponsePropertiesPromptCacheRetentionTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ModelResponsePropertiesPromptCacheRetentionTextVariantEnum::InMemory => "in-memory",
            ModelResponsePropertiesPromptCacheRetentionTextVariantEnum::Variant24h => "24h",
        };
        write!(f, "{}", value)
    }
}
