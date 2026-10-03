use serde::{Deserialize, Serialize};

/// UsageWebSearchCallsResultApiSourceTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UsageWebSearchCallsResultApiSourceTextVariantEnum {
    #[serde(rename = "agents_api")]
    AgentsApi,
    Unlabeled,
}

impl Default for UsageWebSearchCallsResultApiSourceTextVariantEnum {
    fn default() -> Self {
        Self::AgentsApi
    }
}

impl std::fmt::Display for UsageWebSearchCallsResultApiSourceTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            UsageWebSearchCallsResultApiSourceTextVariantEnum::AgentsApi => "agents_api",
            UsageWebSearchCallsResultApiSourceTextVariantEnum::Unlabeled => "unlabeled",
        };
        write!(f, "{}", value)
    }
}
