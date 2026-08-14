use serde::{Deserialize, Serialize};

/// BetaPersonalityEnumTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaPersonalityEnumTextVariantEnum {
    Friendly,
    Pragmatic,
}

impl Default for BetaPersonalityEnumTextVariantEnum {
    fn default() -> Self {
        Self::Friendly
    }
}

impl std::fmt::Display for BetaPersonalityEnumTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaPersonalityEnumTextVariantEnum::Friendly => "friendly",
            BetaPersonalityEnumTextVariantEnum::Pragmatic => "pragmatic",
        };
        write!(f, "{}", value)
    }
}
