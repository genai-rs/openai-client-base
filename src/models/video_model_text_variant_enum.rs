use serde::{Deserialize, Serialize};

/// VideoModelTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoModelTextVariantEnum {
    #[serde(rename = "sora-2")]
    Sora2,
    #[serde(rename = "sora-2-pro")]
    Sora2Pro,
    #[serde(rename = "sora-2-2025-10-06")]
    Sora220251006,
    #[serde(rename = "sora-2-pro-2025-10-06")]
    Sora2Pro20251006,
    #[serde(rename = "sora-2-2025-12-08")]
    Sora220251208,
}

impl Default for VideoModelTextVariantEnum {
    fn default() -> Self {
        Self::Sora2
    }
}

impl std::fmt::Display for VideoModelTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            VideoModelTextVariantEnum::Sora2 => "sora-2",
            VideoModelTextVariantEnum::Sora2Pro => "sora-2-pro",
            VideoModelTextVariantEnum::Sora220251006 => "sora-2-2025-10-06",
            VideoModelTextVariantEnum::Sora2Pro20251006 => "sora-2-pro-2025-10-06",
            VideoModelTextVariantEnum::Sora220251208 => "sora-2-2025-12-08",
        };
        write!(f, "{}", value)
    }
}
