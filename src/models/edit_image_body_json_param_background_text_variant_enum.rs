use serde::{Deserialize, Serialize};

/// EditImageBodyJsonParamBackgroundTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EditImageBodyJsonParamBackgroundTextVariantEnum {
    Transparent,
    Opaque,
    Auto,
}

impl Default for EditImageBodyJsonParamBackgroundTextVariantEnum {
    fn default() -> Self {
        Self::Transparent
    }
}

impl std::fmt::Display for EditImageBodyJsonParamBackgroundTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            EditImageBodyJsonParamBackgroundTextVariantEnum::Transparent => "transparent",
            EditImageBodyJsonParamBackgroundTextVariantEnum::Opaque => "opaque",
            EditImageBodyJsonParamBackgroundTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}
