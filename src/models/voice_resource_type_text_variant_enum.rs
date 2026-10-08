use serde::{Deserialize, Serialize};

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
