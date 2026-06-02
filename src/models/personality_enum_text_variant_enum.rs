use serde::{Deserialize, Serialize};

/// PersonalityEnumTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PersonalityEnumTextVariantEnum {
    Friendly,
    Pragmatic,
}

impl Default for PersonalityEnumTextVariantEnum {
    fn default() -> Self {
        Self::Friendly
    }
}

impl std::fmt::Display for PersonalityEnumTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            PersonalityEnumTextVariantEnum::Friendly => "friendly",
            PersonalityEnumTextVariantEnum::Pragmatic => "pragmatic",
        };
        write!(f, "{}", value)
    }
}
