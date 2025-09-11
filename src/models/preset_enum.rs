use serde::{Deserialize, Serialize};

/// PresetEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PresetEnum {
    #[serde(rename = "babbage-002")]
    Babbage002,
    #[serde(rename = "davinci-002")]
    Davinci002,
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt3_5Turbo,
    #[serde(rename = "gpt-4o-mini")]
    Gpt4oMini,
}

impl Default for PresetEnum {
    fn default() -> Self {
        Self::Babbage002
    }
}
