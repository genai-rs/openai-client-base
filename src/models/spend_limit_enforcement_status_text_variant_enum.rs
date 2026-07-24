use serde::{Deserialize, Serialize};

/// SpendLimitEnforcementStatusTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SpendLimitEnforcementStatusTextVariantEnum {
    Inactive,
    Enforcing,
}

impl Default for SpendLimitEnforcementStatusTextVariantEnum {
    fn default() -> Self {
        Self::Inactive
    }
}

impl std::fmt::Display for SpendLimitEnforcementStatusTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            SpendLimitEnforcementStatusTextVariantEnum::Inactive => "inactive",
            SpendLimitEnforcementStatusTextVariantEnum::Enforcing => "enforcing",
        };
        write!(f, "{}", value)
    }
}
