use serde::{Deserialize, Serialize};

/// ServiceTierResponsesTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceTierResponsesTextVariantEnum {
    Auto,
    Default,
    Flex,
    Scale,
    Priority,
    Fast,
    Ultrafast,
}

impl Default for ServiceTierResponsesTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for ServiceTierResponsesTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ServiceTierResponsesTextVariantEnum::Auto => "auto",
            ServiceTierResponsesTextVariantEnum::Default => "default",
            ServiceTierResponsesTextVariantEnum::Flex => "flex",
            ServiceTierResponsesTextVariantEnum::Scale => "scale",
            ServiceTierResponsesTextVariantEnum::Priority => "priority",
            ServiceTierResponsesTextVariantEnum::Fast => "fast",
            ServiceTierResponsesTextVariantEnum::Ultrafast => "ultrafast",
        };
        write!(f, "{}", value)
    }
}
