use serde::{Deserialize, Serialize};

/// CostsResultApiSourceTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CostsResultApiSourceTextVariantEnum {
    #[serde(rename = "agents_api")]
    AgentsApi,
    Unlabeled,
}

impl Default for CostsResultApiSourceTextVariantEnum {
    fn default() -> Self {
        Self::AgentsApi
    }
}

impl std::fmt::Display for CostsResultApiSourceTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            CostsResultApiSourceTextVariantEnum::AgentsApi => "agents_api",
            CostsResultApiSourceTextVariantEnum::Unlabeled => "unlabeled",
        };
        write!(f, "{}", value)
    }
}
