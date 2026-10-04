use serde::{Deserialize, Serialize};

/// CreateImageRequestBackgroundTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreateImageRequestBackgroundTextVariantEnum {
    Transparent,
    Opaque,
    Auto,
}

impl Default for CreateImageRequestBackgroundTextVariantEnum {
    fn default() -> Self {
        Self::Transparent
    }
}

impl std::fmt::Display for CreateImageRequestBackgroundTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            CreateImageRequestBackgroundTextVariantEnum::Transparent => "transparent",
            CreateImageRequestBackgroundTextVariantEnum::Opaque => "opaque",
            CreateImageRequestBackgroundTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
