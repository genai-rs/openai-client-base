use crate::models;
use serde::{Deserialize, Serialize};

/// AssistantsApiResponseFormatOption - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssistantsApiResponseFormatOption {
    TextVariant(TextVariantEnum),
    Responseformattext(models::ResponseFormatText),
    Responseformatjsonobject(models::ResponseFormatJsonObject),
    Responseformatjsonschema(models::ResponseFormatJsonSchema),
}

/// TextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TextVariantEnum {
    #[serde(rename = "gpt-5")]
    Gpt5,
    #[serde(rename = "gpt-5-mini")]
    Gpt5Mini,
    #[serde(rename = "gpt-5-nano")]
    Gpt5Nano,
    #[serde(rename = "gpt-5-2025-08-07")]
    Gpt520250807,
    #[serde(rename = "gpt-5-mini-2025-08-07")]
    Gpt5Mini20250807,
    #[serde(rename = "gpt-5-nano-2025-08-07")]
    Gpt5Nano20250807,
    #[serde(rename = "gpt-4.1")]
    Gpt4.1,
    #[serde(rename = "gpt-4.1-mini")]
    Gpt4.1Mini,
    #[serde(rename = "gpt-4.1-nano")]
    Gpt4.1Nano,
    #[serde(rename = "gpt-4.1-2025-04-14")]
    Gpt4.120250414,
    #[serde(rename = "gpt-4.1-mini-2025-04-14")]
    Gpt4.1Mini20250414,
    #[serde(rename = "gpt-4.1-nano-2025-04-14")]
    Gpt4.1Nano20250414,
    #[serde(rename = "gpt-4o")]
    Gpt4o,
    #[serde(rename = "gpt-4o-2024-11-20")]
    Gpt4o20241120,
    #[serde(rename = "gpt-4o-2024-08-06")]
    Gpt4o20240806,
    #[serde(rename = "gpt-4o-2024-05-13")]
    Gpt4o20240513,
    #[serde(rename = "gpt-4o-mini")]
    Gpt4oMini,
    #[serde(rename = "gpt-4o-mini-2024-07-18")]
    Gpt4oMini20240718,
    #[serde(rename = "gpt-4.5-preview")]
    Gpt4.5Preview,
    #[serde(rename = "gpt-4.5-preview-2025-02-27")]
    Gpt4.5Preview20250227,
    #[serde(rename = "gpt-4-turbo")]
    Gpt4Turbo,
    #[serde(rename = "gpt-4-turbo-2024-04-09")]
    Gpt4Turbo20240409,
    #[serde(rename = "gpt-4-0125-preview")]
    Gpt40125Preview,
    #[serde(rename = "gpt-4-turbo-preview")]
    Gpt4TurboPreview,
    #[serde(rename = "gpt-4-1106-preview")]
    Gpt41106Preview,
    #[serde(rename = "gpt-4-vision-preview")]
    Gpt4VisionPreview,
    #[serde(rename = "gpt-4")]
    Gpt4,
    #[serde(rename = "gpt-4-0314")]
    Gpt40314,
    #[serde(rename = "gpt-4-0613")]
    Gpt40613,
    #[serde(rename = "gpt-4-32k")]
    Gpt432k,
    #[serde(rename = "gpt-4-32k-0314")]
    Gpt432k0314,
    #[serde(rename = "gpt-4-32k-0613")]
    Gpt432k0613,
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt3.5Turbo,
    #[serde(rename = "gpt-3.5-turbo-16k")]
    Gpt3.5Turbo16k,
    #[serde(rename = "gpt-3.5-turbo-0613")]
    Gpt3.5Turbo0613,
    #[serde(rename = "gpt-3.5-turbo-1106")]
    Gpt3.5Turbo1106,
    #[serde(rename = "gpt-3.5-turbo-0125")]
    Gpt3.5Turbo0125,
    #[serde(rename = "gpt-3.5-turbo-16k-0613")]
    Gpt3.5Turbo16k0613,
}

impl Default for TextVariantEnum {
    fn default() -> Self {
        Self::Gpt5
    }
}
