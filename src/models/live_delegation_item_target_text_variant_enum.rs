use serde::{Deserialize, Serialize};

/// LiveDelegationItemTargetTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LiveDelegationItemTargetTextVariantEnum {
    Decisions,
}

impl Default for LiveDelegationItemTargetTextVariantEnum {
    fn default() -> Self {
        Self::Decisions
    }
}

impl std::fmt::Display for LiveDelegationItemTargetTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            LiveDelegationItemTargetTextVariantEnum::Decisions => "decisions",
        };
        write!(f, "{}", value)
    }
}
