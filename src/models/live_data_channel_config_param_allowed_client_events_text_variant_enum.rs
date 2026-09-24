use serde::{Deserialize, Serialize};

/// LiveDataChannelConfigParamAllowedClientEventsTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LiveDataChannelConfigParamAllowedClientEventsTextVariantEnum {
    All,
}

impl Default for LiveDataChannelConfigParamAllowedClientEventsTextVariantEnum {
    fn default() -> Self {
        Self::All
    }
}

impl std::fmt::Display for LiveDataChannelConfigParamAllowedClientEventsTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            LiveDataChannelConfigParamAllowedClientEventsTextVariantEnum::All => "all",
        };
        write!(f, "{}", value)
    }
}
