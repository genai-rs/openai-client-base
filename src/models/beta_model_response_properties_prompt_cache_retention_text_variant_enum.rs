use serde::{Deserialize, Serialize};

/// BetaModelResponsePropertiesPromptCacheRetentionTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaModelResponsePropertiesPromptCacheRetentionTextVariantEnum {
    #[serde(rename = "in_memory")]
    InMemory,
    #[serde(rename = "24h")]
    Variant24h,
}

impl Default for BetaModelResponsePropertiesPromptCacheRetentionTextVariantEnum {
    fn default() -> Self {
        Self::InMemory
    }
}

impl std::fmt::Display for BetaModelResponsePropertiesPromptCacheRetentionTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaModelResponsePropertiesPromptCacheRetentionTextVariantEnum::InMemory => "in_memory",
            BetaModelResponsePropertiesPromptCacheRetentionTextVariantEnum::Variant24h => "24h",
        };
        write!(f, "{}", value)
    }
}
