use serde::{Deserialize, Serialize};

/// VideoModel - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VideoModel {
    Text(String),
    TextVariant(VideoModelTextVariantEnum),
}

impl Default for VideoModel {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl VideoModel {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for VideoModel {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for VideoModel {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for VideoModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoModel::Text(value) => write!(f, "{}", value),
            VideoModel::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

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
