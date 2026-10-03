use serde::{Deserialize, Serialize};

/// UsageCompletionsResultApiSourceTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UsageCompletionsResultApiSourceTextVariantEnum {
    #[serde(rename = "agents_api")]
    AgentsApi,
    Unlabeled,
}

impl Default for UsageCompletionsResultApiSourceTextVariantEnum {
    fn default() -> Self {
        Self::AgentsApi
    }
}

impl std::fmt::Display for UsageCompletionsResultApiSourceTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            UsageCompletionsResultApiSourceTextVariantEnum::AgentsApi => "agents_api",
            UsageCompletionsResultApiSourceTextVariantEnum::Unlabeled => "unlabeled",
        };
        write!(f, "{}", value)
    }
}
