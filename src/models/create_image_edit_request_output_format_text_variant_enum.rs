use serde::{Deserialize, Serialize};

/// CreateImageEditRequestOutputFormatTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreateImageEditRequestOutputFormatTextVariantEnum {
    Png,
    Jpeg,
    Webp,
}

impl Default for CreateImageEditRequestOutputFormatTextVariantEnum {
    fn default() -> Self {
        Self::Png
    }
}

impl std::fmt::Display for CreateImageEditRequestOutputFormatTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            CreateImageEditRequestOutputFormatTextVariantEnum::Png => "png",
            CreateImageEditRequestOutputFormatTextVariantEnum::Jpeg => "jpeg",
            CreateImageEditRequestOutputFormatTextVariantEnum::Webp => "webp",
        };
        write!(f, "{}", value)
    }
}
