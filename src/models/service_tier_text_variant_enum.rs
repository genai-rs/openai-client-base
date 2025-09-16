use serde::{Deserialize, Serialize};

/// ServiceTierTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceTierTextVariantEnum {
    Auto,
    Default,
    Flex,
    Scale,
    Priority,
}

impl Default for ServiceTierTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for ServiceTierTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ServiceTierTextVariantEnum::Auto => "auto",
            ServiceTierTextVariantEnum::Default => "default",
            ServiceTierTextVariantEnum::Flex => "flex",
            ServiceTierTextVariantEnum::Scale => "scale",
            ServiceTierTextVariantEnum::Priority => "priority",
        };
        write!(f, "{}", value)
    }
}
