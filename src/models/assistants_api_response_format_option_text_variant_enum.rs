use serde::{Deserialize, Serialize};

/// AssistantsApiResponseFormatOptionTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssistantsApiResponseFormatOptionTextVariantEnum {
    Auto,
}

impl Default for AssistantsApiResponseFormatOptionTextVariantEnum {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for AssistantsApiResponseFormatOptionTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            AssistantsApiResponseFormatOptionTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
