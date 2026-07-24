use crate::models;
use serde::{Deserialize, Serialize};

/// SpendLimitEnforcementStatus - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpendLimitEnforcementStatus {
    Text(String),
    TextVariant(SpendLimitEnforcementStatusTextVariantEnum),
}

impl Default for SpendLimitEnforcementStatus {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl SpendLimitEnforcementStatus {
    pub fn new_text(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<String> for SpendLimitEnforcementStatus {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for SpendLimitEnforcementStatus {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}
impl std::fmt::Display for SpendLimitEnforcementStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpendLimitEnforcementStatus::Text(value) => write!(f, "{}", value),
            SpendLimitEnforcementStatus::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

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
