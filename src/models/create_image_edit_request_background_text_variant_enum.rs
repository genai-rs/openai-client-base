use serde::{Deserialize, Serialize};

/// CreateImageEditRequestBackgroundTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreateImageEditRequestBackgroundTextVariantEnum {
    Transparent,
    Opaque,
    Auto,
}

impl Default for CreateImageEditRequestBackgroundTextVariantEnum {
    fn default() -> Self {
        Self::Transparent
    }
}

impl std::fmt::Display for CreateImageEditRequestBackgroundTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            CreateImageEditRequestBackgroundTextVariantEnum::Transparent => "transparent",
            CreateImageEditRequestBackgroundTextVariantEnum::Opaque => "opaque",
            CreateImageEditRequestBackgroundTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
