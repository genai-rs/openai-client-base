use crate::models;
use serde::{Deserialize, Serialize};

/// VoiceResourceType - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VoiceResourceType {
    Text(String),
    TextVariant(VoiceResourceTypeTextVariantEnum),
}

impl Default for VoiceResourceType {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl VoiceResourceType {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for VoiceResourceType {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for VoiceResourceType {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for VoiceResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VoiceResourceType::Text(value) => write!(f, "{}", value),
            VoiceResourceType::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

/// VoiceResourceTypeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VoiceResourceTypeTextVariantEnum {
    #[serde(rename = "audio_sample")]
    AudioSample,
}

impl Default for VoiceResourceTypeTextVariantEnum {
    fn default() -> Self {
        Self::AudioSample
    }
}

impl std::fmt::Display for VoiceResourceTypeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            VoiceResourceTypeTextVariantEnum::AudioSample => "audio_sample",
        };
        write!(f, "{}", value)
    }
}
