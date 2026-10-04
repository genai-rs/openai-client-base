use serde::{Deserialize, Serialize};

/// CreateImageRequestOutputFormatTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreateImageRequestOutputFormatTextVariantEnum {
    Png,
    Jpeg,
    Webp,
}

impl Default for CreateImageRequestOutputFormatTextVariantEnum {
    fn default() -> Self {
        Self::Png
    }
}

impl std::fmt::Display for CreateImageRequestOutputFormatTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            CreateImageRequestOutputFormatTextVariantEnum::Png => "png",
            CreateImageRequestOutputFormatTextVariantEnum::Jpeg => "jpeg",
            CreateImageRequestOutputFormatTextVariantEnum::Webp => "webp",
        };
        write!(f, "{}", value)
    }
}
