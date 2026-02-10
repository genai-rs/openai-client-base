use serde::{Deserialize, Serialize};

/// EditImageBodyJsonParamOutputFormatTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EditImageBodyJsonParamOutputFormatTextVariantEnum {
    Png,
    Jpeg,
    Webp,
}

impl Default for EditImageBodyJsonParamOutputFormatTextVariantEnum {
    fn default() -> Self {
        Self::Png
    }
}

impl std::fmt::Display for EditImageBodyJsonParamOutputFormatTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            EditImageBodyJsonParamOutputFormatTextVariantEnum::Png => "png",
            EditImageBodyJsonParamOutputFormatTextVariantEnum::Jpeg => "jpeg",
            EditImageBodyJsonParamOutputFormatTextVariantEnum::Webp => "webp",
        };
        write!(f, "{}", value)
    }
}
